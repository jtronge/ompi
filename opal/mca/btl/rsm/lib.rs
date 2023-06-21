/// The component type is defined in C
use std::os::raw::c_int;
use std::sync::{Arc, Mutex};
use std::cell::RefCell;
use std::collections::HashMap;
use log::{info, debug};
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
use shared::{BLOCK_SIZE, SharedRegionMap, SharedRegionHandle, BlockID, make_path};
use fifo::FIFO;
use block_store::BlockStore;
use local_data::LocalData;

extern "C" {
    pub static mut mca_btl_rsm: mca_btl_rsm_t;
    pub static mut mca_btl_rsm_component: mca_btl_base_component_3_0_0_t;
}

pub type Rank = u32;

#[derive(Clone, Debug)]
pub enum Error {
    /// Out of memory
    OOM,
    /// Shared memory error
    SharedMemoryFailure,
    /// Failed to lock a data structure
    LockError,
}

pub type Result<T> = std::result::Result<T, Error>;

/// Initialize the RSM component.
#[no_mangle]
unsafe extern "C" fn mca_btl_rsm_component_init(
    num_btls: *mut c_int,
    _enable_progress_threads: bool,
    _enable_mpi_threads: bool,
) -> *mut *mut mca_btl_base_module_t {
    *num_btls = 0;

    // Initialize logging (this would be better with a special ompi or opal implementation)
    env_logger::init();
    info!("running init");

    // Create the shared memory for this rank
    // TODO: Add jobid
    let local_rank = proc_info::local_rank();
    let mut map = SharedRegionMap { regions: HashMap::new() };
    let path = make_path(&proc_info::node_name(), local_rank);
    let region = RefCell::new(SharedRegionHandle::create(path).unwrap());
    map.regions.insert(local_rank, region);
    let map = Arc::new(Mutex::new(map));
    let fifo = FIFO::new(Arc::clone(&map), local_rank);
    let block_store = BlockStore::new(Arc::clone(&map));

    let ptr = (&mut mca_btl_rsm as *mut _) as *mut mca_btl_base_module_t;
    local_data::init(ptr, map, fifo, block_store);
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
    info!("running progress");
    let ptr = (&mut mca_btl_rsm as *mut _) as *mut mca_btl_base_module_t;
    local_data::lock(ptr, |data| {
        // Progress pending outgoing blocks
        while let Some((endpoint_rank, block_id)) = data.pending.pop() {
            let rank: usize = endpoint_rank.try_into().unwrap();
            let endpoint: *mut Endpoint = data.endpoints[rank];
            (*endpoint).fifo.push(proc_info::local_rank(), block_id).unwrap();
        }

        // Poll my local fifo
        let mut count = 0;
        while let Some((endpoint_rank, block_id)) = data.fifo.pop() {
            let rank: usize = endpoint_rank.try_into().unwrap();
            let endpoint: *mut Endpoint = data.endpoints[rank];
            debug!("Popped block {} from fifo", block_id);

            if handle_incoming(data, endpoint, block_id) {
                // Now the block is complete, so we return it
                (*endpoint).fifo.push(endpoint_rank, block_id).unwrap();
                count += 1;
            }
            // See mca_btl_sm_poll_handle_frag in btl/sm
        }
        // Number of blocks received
        count
    })
}

/// Handle an incoming block. Return true if the block needs to be returned to
/// the sender, and false if this was a block being returned to this rank.
unsafe fn handle_incoming(
    data: &mut LocalData,
    endpoint: *mut Endpoint,
    block_id: BlockID,
) -> bool {
    // TODO: this might be better as a try_lock?
    data.map.lock().unwrap().region_mut((*endpoint).rank, |region| {
        let block_idx: usize = block_id.try_into().unwrap();
        let block = &mut region.blocks[block_idx];

        // Free returned blocks
        if block.complete && (*endpoint).rank == proc_info::local_rank() {
            block.complete = false;
            data.block_store.free(block_id);
            return false;
        }

        let idx = block.message_trigger;
        let reg: mca_btl_active_message_callback_t = mca_btl_base_active_message_trigger[idx];
        let segments = [
            mca_btl_base_segment_t {
                seg_addr: opal_ptr_t {
                    pval: block.data.as_mut_ptr() as *mut _,
                },
                seg_len: block.data.len().try_into().unwrap(),
            },
        ];
        let mut recv_de = mca_btl_base_receive_descriptor_t {
            endpoint: endpoint as *mut _,
            des_segments: segments.as_ptr(),
            des_segment_count: segments.len(),
            tag: block.tag,
            cbdata: reg.cbdata,
        };

        // Handle fragment, call the receive callback
        reg.cbfunc.unwrap()(
            (&mut mca_btl_rsm as *mut _) as *mut mca_btl_base_module_t,
            &mut recv_de,
        );

        // Set the block to complete
        block.complete = true;

        // The block needs to be returned to sender
        true
    })
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
