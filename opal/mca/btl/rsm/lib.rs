/// The component type is defined in C
use std::os::raw::c_int;
use std::sync::Mutex;
#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(improper_ctypes)]
mod opal;
mod module;
mod modex;
mod proc_info;
mod shared_mem;
mod module_data;
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
use shared_mem::{SharedMemory, SharedMemoryOptions};

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

    let shmem = SharedMemory::create(SharedMemoryOptions {
        backing_directory: "/tmp".to_string(),
        num_local_peers: proc_info::num_local_peers(),
        node_name: proc_info::node_name(),
        node_rank: proc_info::node_rank(),
        // TODO: Not sure where to get the jobid/euid from?
        euid: 0,
        jobid: 0,
    }).unwrap();

    let ptr = (&mut mca_btl_rsm as *mut _) as *mut mca_btl_base_module_t;
    module_data::init(ptr, shmem);
    *btls = ptr;
    btls
}

#[no_mangle]
unsafe extern "C" fn mca_btl_rsm_component_progress() -> c_int {
    // Progress endpoints
    let ptr = (&mut mca_btl_rsm as *mut _) as *mut mca_btl_base_module_t;
    module_data::lock(ptr, |data| {
        while let Some(mut pblock) = data.pending.pop() {
            data.shmem.lock_fifo(pblock.local_rank.try_into().unwrap(), |fifo| {
                fifo.push(pblock.block.take().unwrap());
            });
        }
        // Poll my local fifo
        data.shmem.lock_fifo(proc_info::local_rank().try_into().unwrap(), |fifo| {
            while let Some(mut block) = fifo.pop() {
                let idx: usize = block.tag().into();
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

                reg.cbfunc.unwrap()(
                    (&mut mca_btl_rsm as *mut _) as *mut mca_btl_base_module_t,
                    &mut recv_de,
                );
                // See mca_btl_sm_poll_handle_frag in btl/sm
                // Handle fragment, call the receive callback
            }
            // Return number of blocks received
            0
        })
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
