/// The component type is defined in C
use std::os::raw::c_int;
use std::path::PathBuf;
use std::sync::Mutex;
use log::{info, debug};
use shared_memory::ShmemConf;
#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(improper_ctypes)]
mod opal;
mod shared;
mod endpoint;
mod module;
mod modex;
mod proc_info;
// mod shared_mem;
mod local_data;
use opal::{
    mca_btl_base_module_t,
    mca_base_component_var_register,
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
// use shared_mem::{SharedMemory, SharedMemoryOptions};
use endpoint::Endpoint;
use shared::{SHARED_MEM_SIZE, FIFO};

extern "C" {
    pub static mut mca_btl_rsm: mca_btl_rsm_t;
    pub static mut mca_btl_rsm_component: mca_btl_base_component_3_0_0_t;
}

#[derive(Clone, Debug)]
pub enum Error {
    /// Out of memory
    OOM,
}

pub type Result<T> = std::result::Result<T, Error>;

/// Initialize the RSM component.
#[no_mangle]
unsafe extern "C" fn mca_btl_rsm_component_init(
    num_btls: *mut c_int,
    enable_progress_threads: bool,
    enable_mpi_threads: bool,
) -> *mut *mut mca_btl_base_module_t {
    *num_btls = 0;

    // Initialize logging (this would be better with a special ompi or opal implementation)
    env_logger::init();
    info!("running init");

    // Create the shared memory for this rank
    // TODO: Add jobid
    let fname = format!("{}-{}.shmem", proc_info::node_name(), proc_info::local_rank());
    let mut path = PathBuf::new();
    path.push("/dev/shm");
    path.push(fname);
    let shmem = ShmemConf::new()
        .size(SHARED_MEM_SIZE)
        .flink(path)
        .create()
        .unwrap();
    let ptr = shmem.as_ptr();
    std::ptr::write_bytes(ptr, 0, SHARED_MEM_SIZE);
    FIFO::init(ptr as *mut _);

    let ptr = (&mut mca_btl_rsm as *mut _) as *mut mca_btl_base_module_t;
    local_data::init(ptr, shmem);
    *num_btls = 1;
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

    *btls = ptr;
    btls

/*
    let shmem = SharedMemory::create(SharedMemoryOptions {
        backing_directory: "/tmp".to_string(),
        num_local_peers: proc_info::num_local_peers(),
        node_name: proc_info::node_name(),
        node_rank: proc_info::node_rank(),
        // TODO: Not sure where to get the jobid/euid from?
        euid: 0,
        // Extract from opal_proc_t
        jobid: 0,
    }).unwrap();

    let ptr = (&mut mca_btl_rsm as *mut _) as *mut mca_btl_base_module_t;
    local_data::init(ptr, shmem);
    *num_btls = 1;
    *btls = ptr;
    btls
*/
}

#[no_mangle]
unsafe extern "C" fn mca_btl_rsm_component_progress() -> c_int {
    info!("running progress");
    // Progress endpoints
    let ptr = (&mut mca_btl_rsm as *mut _) as *mut mca_btl_base_module_t;
    local_data::lock(ptr, |data| {
        // Progress pending endpoints
        while let Some((endpoint_rank, block_id)) = data.pending.pop() {
            let rank: usize = endpoint_rank.try_into().unwrap();
            let endpoint: *mut Endpoint = data.endpoints[rank];
            (*endpoint).push(block_id).unwrap();
        }
        let mut count = 0;
        while let Some((endpoint_rank, block_id)) = data.pop() {
            let rank: usize = endpoint_rank.try_into().unwrap();
            let endpoint: *mut Endpoint = data.endpoints[rank];
            debug!("Popped block {} from fifo", block_id);
            (*endpoint).use_block(block_id, |block| {
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
                // TODO: this might be better as a try_lock?
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
                count += 1;
                // See mca_btl_sm_poll_handle_frag in btl/sm
            });
        }
        count
        // Poll my local fifo
/*
        data.shmem.lock_fifo(proc_info::local_rank().try_into().unwrap(), |fifo| {
            let mut count = 0;
            while let Some(mut block) = fifo.pop() {
                let idx: usize = block.tag().into();
                debug!("Popped block {} from fifo", idx);
                let reg: mca_btl_active_message_callback_t = mca_btl_base_active_message_trigger[idx];
                let segments = [
                    mca_btl_base_segment_t {
                        seg_addr: opal_ptr_t {
                            pval: block.as_mut() as *mut _,
                        },
                        seg_len: block.len().try_into().unwrap(),
                    },
                ];
                // TODO: this might be better as a try_lock?
                let endpoint = *data.endpoints
                    .iter()
                    .find(|&ep| (**ep).local_rank == block.src())
                    .expect("Failed to find endpoint");
                let mut recv_de = mca_btl_base_receive_descriptor_t {
                    endpoint: endpoint as *mut _,
                    des_segments: segments.as_ptr(),
                    des_segment_count: segments.len(),
                    tag: block.tag(),
                    cbdata: reg.cbdata,
                };

                // Handle fragment, call the receive callback
                reg.cbfunc.unwrap()(
                    (&mut mca_btl_rsm as *mut _) as *mut mca_btl_base_module_t,
                    &mut recv_de,
                );
                count += 1;
                // See mca_btl_sm_poll_handle_frag in btl/sm
            }
            // Return number of blocks received
            count
        })
*/
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

const MAX_EAGER_LIMIT: usize = 4 * 1024;
const MAX_RNDV_EAGER_LIMIT: usize = 32 * 1024;
const MAX_SEND_SIZE: usize = 32 * 1024;

#[no_mangle]
unsafe extern "C" fn mca_btl_rsm_component_register_params() -> c_int {
    mca_btl_rsm.parent.btl_eager_limit = MAX_EAGER_LIMIT;
    mca_btl_rsm.parent.btl_rndv_eager_limit = MAX_RNDV_EAGER_LIMIT;
    mca_btl_rsm.parent.btl_max_send_size = MAX_SEND_SIZE;
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
