/// The component type is defined in C
use std::os::raw::c_int;
use std::sync::{Arc, Mutex};
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::Write;
use log::{info, debug};
use shared_memory::ShmemError;
#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(improper_ctypes)]
mod opal;
mod shared;
mod fifo;
mod block_store;
mod endpoint;
mod module;
mod modex;
mod proc_info;
mod local_data;
use opal::{
    mca_btl_base_module_t,
    mca_btl_active_message_callback_t,
    mca_btl_base_active_message_trigger,
    mca_btl_base_module_recv_cb_fn_t,
    mca_btl_base_param_register,
    mca_btl_base_receive_descriptor_t,
    mca_btl_base_component_3_0_0_t,
    mca_btl_base_segment_t,
    mca_btl_rsm_t,
    opal_ptr_t,
    MCA_BTL_FLAGS_SEND_INPLACE,
    MCA_BTL_FLAGS_SEND,
    OPAL_SUCCESS,
    calloc,
};
use endpoint::Endpoint;
use shared::{BLOCK_SIZE, SharedRegionMap, SharedRegionHandle, Descriptor, BlockID, make_path};
use fifo::FIFO;
use block_store::BlockStore;
use local_data::LocalData;

extern "C" {
    pub static mut mca_btl_rsm: mca_btl_rsm_t;
    pub static mut mca_btl_rsm_component: mca_btl_base_component_3_0_0_t;
}

pub type Rank = u32;

#[derive(Debug)]
pub enum Error {
    /// Out of memory
    OOM,
    /// Shared memory error
    SharedMemoryFailure(ShmemError),
    /// Failed to lock a data structure
    LockError,
    /// An error occurred in an opal component that was called
    OpalError(c_int),
    /// An error occurred receiving a modex value
    ModexValueRecvFailure,
}

pub type Result<T> = std::result::Result<T, Error>;

pub const SHARED_MEM_NAME_KEY: &'static str = "rsm.shared_mem_name_key";

/// Initialize the RSM component.
#[no_mangle]
unsafe extern "C" fn mca_btl_rsm_component_init(
    num_btls: *mut c_int,
    _enable_progress_threads: bool,
    _enable_mpi_threads: bool,
) -> *mut *mut mca_btl_base_module_t {
    *num_btls = 0;

    // Initialize logging (this would be better with a special ompi or opal implementation)
    env_logger::builder()
        .format(|buf, record| {
            writeln!(buf, "(rank = {}) {}: {}", proc_info::local_rank(), record.level(), record.args())
        })
        .init();

    // Create the shared memory for this rank
    // TODO: Add jobid
    let local_rank = proc_info::local_rank();
    let mut map = SharedRegionMap { regions: HashMap::new() };
    let path = make_path(proc_info::node_name(), local_rank, std::process::id());
    // Publish the path
    match modex::send_string_local(SHARED_MEM_NAME_KEY, path.as_os_str().to_str().unwrap()) {
        Ok(()) => (),
        Err(err) => {
            debug!("Modex error: {:?}", err);
            return std::ptr::null_mut();
        }
    }

    let region = match SharedRegionHandle::create(path) {
        Ok(region) => region,
        Err(err) => {
            debug!("Shared memory error: {:?}", err);
            return std::ptr::null_mut();
        }
    };
    let region = RefCell::new(region);
    map.regions.insert(local_rank, region);
    let map = Arc::new(Mutex::new(map));
    let fifo = FIFO::new(Arc::clone(&map), local_rank);
    let block_store = BlockStore::new(Arc::clone(&map));

    let ptr = (&mut mca_btl_rsm as *mut _) as *mut mca_btl_base_module_t;
    local_data::init(ptr, map, fifo, block_store);
    // Create a self endpoint
    match local_data::lock(ptr, |data| {
        let endpoint = Endpoint::new(Arc::clone(&data.map), proc_info::local_rank())?;
        let endpoint_ptr = Box::into_raw(Box::new(endpoint));
        info!("my endpoint pointer: {}", endpoint_ptr as usize);
        data.endpoints.push(endpoint_ptr);
        Ok::<(), Error>(())
    }) {
        Ok(()) => (),
        Err(_) => return std::ptr::null_mut(),
    };
    // Have to allocate using calloc since this will be freed in the base btl
    // code
    let btls = calloc(
        1,
        // Assume this will never be bigger than about 8-16 bytes
        std::mem::size_of::<*mut mca_btl_base_module_t>().try_into().unwrap(),
    ) as *mut *mut mca_btl_base_module_t;
    if btls.is_null() {
        return std::ptr::null_mut();
    }

    *num_btls = 1;
    *btls = ptr;
    btls
}

#[no_mangle]
unsafe extern "C" fn mca_btl_rsm_component_progress() -> c_int {
    // TODO: Use of below pointer could very well be UB
    let btl = (&mut mca_btl_rsm as *mut _) as *mut mca_btl_base_module_t;
    local_data::lock(btl, |data| {
        // Progress pending outgoing blocks
        while let Some((endpoint_rank, block_id)) = data.pending.pop() {
            let endpoint: *mut Endpoint = *data.endpoints
                .iter()
                .find(|ep| (*(*(*ep))).rank == endpoint_rank.into())
                .unwrap();
            info!("Pushing pending block: {}", block_id);
            (*endpoint).fifo.push(proc_info::local_rank(), block_id).unwrap();
        }
    });

    // Poll my local fifo
    let mut count = 0;
    loop {
        let handler = local_data::lock(btl, |data| {
            if let Some((endpoint_rank, block_id)) = data.fifo.pop() {
                let endpoint: *mut Endpoint = *data.endpoints
                    .iter()
                    .find(|ep| (*(*(*ep))).rank == endpoint_rank)
                    .unwrap();

                Some((handle_incoming(data, endpoint, endpoint_rank, block_id), endpoint))
            } else {
                None
            }
        });

        // See mca_btl_sm_poll_handle_frag in btl/sm
        if let Some((mut handler, endpoint)) = handler {
            if handler.run(btl) {
                info!("Pushing complete block: ({}, {})", handler.rank, handler.block_id);
                // Now the block is complete, so we return it
                (*endpoint).fifo.push(handler.rank, handler.block_id).unwrap();
                count += 1;
            }
        } else {
            break;
        }
    }
    // Number of blocks received
    count
}

enum HandlerKind {
    CompleteCallback(Option<(*mut Descriptor, *mut Endpoint)>),
    ReceiveCallback(mca_btl_base_module_recv_cb_fn_t, mca_btl_base_receive_descriptor_t),
}

struct Handler {
    rank: Rank,
    block_id: BlockID,
    kind: Option<HandlerKind>,
}

impl Handler {
    /// Run the handler, returning whether or not this block should be returned
    /// (to complete it) to the sending process.
    ///
    /// WARNING: This must not be called while the local_data lock is held, or
    /// deadlock will ensue.
    unsafe fn run(&mut self, btl: *mut mca_btl_base_module_t) -> bool {
        let complete = if let Some(kind) = self.kind.take() {
            match kind {
                HandlerKind::CompleteCallback(Some((desc, endpoint))) => {
                    if let Some(cbfunc) = (*desc).base.des_cbfunc {
                        cbfunc(
                            // (&mut mca_btl_rsm as *mut _) as *mut _,
                            btl,
                            endpoint as *mut _,
                            desc as *mut _,
                            OPAL_SUCCESS,
                        );
                    }
                    // Now destroy the callback
                    local_data::lock(btl, |data| {
                        data.free_descriptor(desc);
                    });
                    false
                }
                HandlerKind::CompleteCallback(None) => false,
                HandlerKind::ReceiveCallback(cbfunc, mut recv_de) => {
                    if let Some(cbfunc) = cbfunc {
                        cbfunc(
                            // (&mut mca_btl_rsm as *mut _) as *mut _,
                            btl,
                            &mut recv_de as *mut _,
                        );
                    }
                    true
                }
            }
        } else {
            false
        };

        // Now lock local_data, the region, set the block complete value, and
        // free the block if necessary.
        local_data::lock(btl, |data| {
            data.map.lock().unwrap().region_mut(self.rank, |region| {
                // Set the complete value
                let block_idx: usize = self.block_id.try_into().unwrap();
                region.blocks[block_idx].complete = complete;
            });

            if !complete {
                // Free the block (this rank sent the block previously)
                assert_eq!(self.rank, proc_info::local_rank());
                data.block_store.free(self.block_id);
            }
        });

        complete
    }
}

/// Handle an incoming block. Return true if the block needs to be returned to
/// the sender, and false if this was a block being returned to this rank.
unsafe fn handle_incoming(
    data: &mut LocalData,
    endpoint: *mut Endpoint,
    rank: Rank,
    block_id: BlockID,
) -> Handler {
    // TODO: this might be better as a try_lock?
    let kind = data.map.lock().unwrap().region_mut(rank, |region| {
        let block_idx: usize = block_id.try_into().unwrap();
        let block = &mut region.blocks[block_idx];
        info!("Handling block: block_id = {}, len = {}, tag = {}, complete = {}, endpoint.rank = {}", block_id, block.len, block.tag, block.complete, (*endpoint).rank);

        // Free returned blocks
        // TODO: Something is wrong with this logic here
        if block.complete {
            data.show_descriptor_info();
            // Find the descriptor
            return if let Some(des) = data.find_descriptor(rank, block_id) {
                Some(HandlerKind::CompleteCallback(Some((des, endpoint))))
            } else {
                info!("Descriptor not found");
                Some(HandlerKind::CompleteCallback(None))
            };
        }

        // Prepare the callback descriptor
        let idx: usize = block.tag.try_into().unwrap();
        let reg: mca_btl_active_message_callback_t = mca_btl_base_active_message_trigger[idx];
        // Segment to be passed to callback (initialized here to avoid drop within block)
        let segment = Box::new(mca_btl_base_segment_t {
            seg_addr: opal_ptr_t {
                pval: block.data.as_mut_ptr() as *mut _,
            },
            seg_len: block.len.try_into().unwrap(),
        });
        let segment_ptr = Box::into_raw(segment);
        let recv_de = mca_btl_base_receive_descriptor_t {
            endpoint: endpoint as *mut _,
            des_segments: segment_ptr as *mut _,
            des_segment_count: 1,
            tag: block.tag,
            cbdata: reg.cbdata,
        };

        // The block needs to be returned to sender
        Some(HandlerKind::ReceiveCallback(reg.cbfunc, recv_de))
    });

    // The callback must be made outside of the lock since it will possibly
    // make a recursive call back into the BTL module which will try to lock
    // again, leading to deadlock if not called here.
    Handler {
        rank,
        block_id,
        kind,
    }
}

#[no_mangle]
unsafe extern "C" fn mca_btl_rsm_component_open() -> c_int {
    OPAL_SUCCESS
}

#[no_mangle]
unsafe extern "C" fn mca_btl_rsm_component_close() -> c_int {
    OPAL_SUCCESS
}

// const MAX_EAGER_LIMIT: usize = 4 * 1024;
// const MAX_RNDV_EAGER_LIMIT: usize = 32 * 1024;
// const MAX_SEND_SIZE: usize = 32 * 1024;

#[no_mangle]
unsafe extern "C" fn mca_btl_rsm_component_register_params() -> c_int {
    // TODO: I'm not sure how these eager/rndv variables will affect usage of
    // this BTL. I see the original sm module has RDMA code, but I think this
    // is out of scope for this implementation.
    mca_btl_rsm.parent.btl_eager_limit = BLOCK_SIZE;
    mca_btl_rsm.parent.btl_rndv_eager_limit = BLOCK_SIZE;

    mca_btl_rsm.parent.btl_max_send_size = BLOCK_SIZE;
    mca_btl_rsm.parent.btl_min_rdma_pipeline_size = i32::MAX.try_into().unwrap();

    mca_btl_rsm.parent.btl_flags = MCA_BTL_FLAGS_SEND_INPLACE | MCA_BTL_FLAGS_SEND;
    mca_btl_rsm.parent.btl_bandwidth = 20000; // Mbs
    mca_btl_rsm.parent.btl_latency = 1; // Microsecs

    mca_btl_base_param_register(
        &mut mca_btl_rsm_component.btl_version,
        (&mut mca_btl_rsm as *mut _) as *mut mca_btl_base_module_t,
    );
    OPAL_SUCCESS
}
