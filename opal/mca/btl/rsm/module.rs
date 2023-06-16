use std::os::raw::{c_int, c_void};
use std::sync::Mutex;
use log::info;
use crate::opal::{
    mca_btl_base_descriptor_t,
    mca_btl_base_endpoint_t,
    mca_btl_base_module_error_cb_fn_t,
    mca_btl_base_module_t,
    mca_btl_base_tag_t,
    opal_bitmap_set_bit,
    opal_bitmap_t,
    opal_convertor_get_current_pointer_rs,
    opal_convertor_pack,
    // TODO: Figure out where these are
    opal_convertor_need_buffers_rs,
    // opal_convertor_on_discrete_device,
    // opal_convertor_on_unified_device,
    opal_convertor_t,
    opal_proc_local_get,
    opal_proc_t,
    opal_proc_on_local_node_rs,
    OPAL_SUCCESS,
    OPAL_ERR_OUT_OF_RESOURCE,
    iovec,
};
use crate::modex::{self, Key};
use crate::proc_info;
use crate::local_data;
use crate::endpoint::Endpoint;

#[repr(C)]
struct Descriptor {
    base: mca_btl_base_descriptor_t,
    block_id: isize,
    endpoint_local_rank: u16,
}

#[no_mangle]
unsafe extern "C" fn mca_btl_rsm_add_procs(
    btl: *mut mca_btl_base_module_t,
    nprocs: usize,
    procs: *mut *mut opal_proc_t,
    peers: *mut *mut mca_btl_base_endpoint_t,
    reachability: *mut opal_bitmap_t,
) -> c_int {
    info!("adding procs");
    let mut rc = 0;
    if reachability.is_null() {
        return 0;
    }
    let my_proc = opal_proc_local_get();
    if my_proc.is_null() {
        return OPAL_ERR_OUT_OF_RESOURCE;
    }
    let nprocs: isize = nprocs.try_into().unwrap();
    for proc in 0..nprocs {
        let proc_data = *(*procs.offset(proc));
        if proc_data.proc_name.jobid != (*my_proc).proc_name.jobid
            || opal_proc_on_local_node_rs(proc_data.proc_flags) == 0 {
            *peers.offset(proc) = std::ptr::null_mut();
            continue;
        }

        if my_proc == *procs.offset(proc) {
            continue;
        }
        // Add procs to accessibility list
        rc = opal_bitmap_set_bit(reachability, proc.try_into().unwrap());

        if rc != OPAL_SUCCESS {
            return rc;
        }

        // Get the local rank of the other process
        let mut local_rank: u16 = 0;
        rc = modex::recv_value(Key::LocalRank, &(*(*procs.offset(proc))).proc_name, &mut local_rank);
        if rc != OPAL_SUCCESS {
            return rc;
        }
        let mut endpoint = Box::new(Endpoint { local_rank });
        let endpoint_ptr = Box::into_raw(endpoint);
        *peers.offset(proc) = endpoint_ptr as *mut _;
        local_data::lock(btl, |data| {
            data.endpoints.push(endpoint_ptr);
        });
        // TODO: Get the message size
        // Now we just need to attach to the shared memory segment
        // TODO: set up endpoint
    }
    rc
}

#[no_mangle]
unsafe extern "C" fn mca_btl_rsm_del_procs(
    btl: *mut mca_btl_base_module_t,
    nprocs: usize,
    procs: *mut *mut opal_proc_t,
    peers: *mut *mut mca_btl_base_endpoint_t,
) -> c_int {
    info!("deleting procs");
    local_data::lock(btl, |data| {
        let nprocs: isize = nprocs.try_into().unwrap();
        for proc in 0..nprocs {
            let peer = peers.offset(proc);
            if !peer.is_null() {
                let ep = peer as *mut Endpoint;
                // Remove it from the endpoints list
                if let Some(i) = data.endpoints
                    .iter()
                    .position(|&other_ep| other_ep == ep)
                {
                    let _ = data.endpoints.swap_remove(i);
                }
                // Convert back to a Box and thus free it
                let ep = Box::from_raw(ep);
                *peer = std::ptr::null_mut();
            }
        }
        OPAL_SUCCESS
    })
}

#[no_mangle]
extern "C" fn mca_btl_rsm_finalize(btl: *mut mca_btl_base_module_t) -> c_int {
    info!("running finalize");
    // TODO: Clean up any resources
    OPAL_SUCCESS
}

/// Allocate a new descriptor and return it.
#[no_mangle]
unsafe extern "C" fn mca_btl_rsm_alloc(
    btl: *mut mca_btl_base_module_t,
    endpoint: *mut mca_btl_base_endpoint_t,
    order: u8,
    size: usize,
    flags: u32,
) -> *mut mca_btl_base_descriptor_t {
    info!("allocating a descriptor of size {}", size);
    local_data::lock(btl, |data| {
        // TODO: Set length
        let block_id = data.alloc();
        Box::into_raw(Box::new(data.descriptor(block_id))) as *mut _
    })
}

/// Free a descriptor.
#[no_mangle]
unsafe extern "C" fn mca_btl_rsm_free(
    btl: *mut mca_btl_base_module_t,
    des: *mut mca_btl_base_descriptor_t,
) -> c_int {
    info!("freeing descriptor");
    // let _ = Box::from_raw(des as *mut Option<Block>);
    let des = Box::from_raw(des as *mut Descriptor);
    local_data::lock(btl, |data| {
        // TODO: Release block
        // data.free(i);
        OPAL_SUCCESS
    })
}

/// Packed data into shared memory.
#[no_mangle]
unsafe extern "C" fn mca_btl_rsm_prepare_src(
    btl: *mut mca_btl_base_module_t,
    endpoint: *mut mca_btl_base_endpoint_t,
    convertor: *mut opal_convertor_t,
    order: u8,
    reserve: usize,
    size: *mut usize,
    flags: u32,
) -> *mut mca_btl_base_descriptor_t {
    info!("calling prepare_src");
    local_data::lock(btl, |data| {
        let block_id = data.alloc();
        let rc = data.use_block(block_id, |block| block.prepare_fill(convertor, reserve, size));
        if rc < 0 {
            return std::ptr::null_mut();
        }
        // TODO: Set order and flags
        let desc = Box::new(data.descriptor(block_id));
        Box::into_raw(desc) as *mut _
/*
        let mut data_ptr: *mut c_void = std::ptr::null_mut();
        opal_convertor_get_current_pointer_rs(convertor, &mut data_ptr);
        assert!(!data_ptr.is_null());
        let mut block = data.shmem.alloc(*size).unwrap();
        block.set_src(proc_info::local_rank());

        // TODO: For now it's always calling opal_convertor_pack(), but we should
        // check for the easy case with opal_convertor_need_buffers()
        let mut iov_count = 1;
        let mut iov = iovec {
            iov_len: *size,
            iov_base: block.as_mut() as *mut _,
        };
        let rc = opal_convertor_pack(convertor, &mut iov, &mut iov_count, size);
        if rc < 0 {
            std::ptr::null_mut()
        } else {
            // if (rc < 0) {
            //  MCA_BTL_SM_FRAG_RETURN(frag)
            //  return NULL;
            // }
            // }

            // The descriptor/fragment being returned should be allocated here and
            // then freed on destruction of the btl
            Box::into_raw(Box::new(Some(block))) as *mut _
        }
*/
    })
}

/// Send a descriptor to the particular endpoint.
#[no_mangle]
unsafe extern "C" fn mca_btl_rsm_send(
    btl: *mut mca_btl_base_module_t,
    endpoint: *mut mca_btl_base_endpoint_t,
    descriptor: *mut mca_btl_base_descriptor_t,
    tag: mca_btl_base_tag_t,
) -> c_int {
    info!("calling send");
    local_data::lock(btl, |data| {
        let endpoint = endpoint as *mut Endpoint;
        let desc = descriptor as *mut Descriptor;
        let block_id = (*desc).block_id;
        data.use_block(block_id, |block| {
            block.tag = tag;
        });
        // The original SM attempts a write into the peer's fifo, here it
        // either writes or fails altogether
        (*endpoint).push(block_id).unwrap();
/*
        let block = descriptor as *mut Option<Block>;
        let mut block = Box::from_raw(block);
        let block_ref = (*block).as_mut().unwrap();
        block_ref.set_tag(tag);
        block_ref.set_src(proc_info::local_rank());
        let pblock = PendingBlock {
            local_rank: (*endpoint).local_rank,
            block,
        };
        data.pending.push(pblock);
*/
        OPAL_SUCCESS
    })
}

/// Do an immediate send to the endpoint.
#[no_mangle]
unsafe extern "C" fn mca_btl_rsm_sendi(
    btl: *mut mca_btl_base_module_t,
    endpoint: *mut mca_btl_base_endpoint_t,
    convertor: *mut opal_convertor_t,
    header: *mut c_void,
    header_size: usize,
    payload_size: usize,
    order: u8,
    flags: u32,
    tag: mca_btl_base_tag_t,
    descriptor: *mut *mut mca_btl_base_descriptor_t,
) -> c_int {
    info!("calling sendi");
    // NOTE: Ignoring data pointer here
    local_data::lock(btl, |data| {
        // TODO: Check pending, return early if there are some
        if data.pending.len() > 0 {
            return OPAL_ERR_OUT_OF_RESOURCE;
        }
        // Alloc block and set output descriptor
        let block_id = data.alloc();
        // Set the block data
        data.use_block(block_id, |block| {
            block.next = -1;
            block.tag = tag;
            block.complete = false;
            block.fill(convertor, header, header_size, payload_size);
        });
        // Set output descriptor
        let desc = Box::new(data.descriptor(block_id));
        *descriptor = Box::into_raw(desc) as *mut _;

        let endpoint = endpoint as *mut Endpoint;
        (*endpoint).push(block_id).unwrap();
/*
        let len = header_size + payload_size;
        let mut block = data.shmem.alloc(len).unwrap();
        block.set_tag(tag);
        block.set_src(proc_info::local_rank());

        std::ptr::copy_nonoverlapping(header as *const u8, block.as_mut(), header_size);
        if payload_size > 0 {
            let mut data_ptr = std::ptr::null_mut();
            opal_convertor_get_current_pointer_rs(convertor, &mut data_ptr);
            let iov_len = 1;

            let mut iov = iovec {
                iov_base: block.as_mut().offset(header_size.try_into().unwrap()) as *mut _,
                iov_len,
            };
            if opal_convertor_need_buffers_rs(convertor) != 0 {
                let mut iov_len: u32 = iov_len.try_into().unwrap();
                let mut length = 0;
                opal_convertor_pack(convertor, &mut iov, &mut iov_len, &mut length);
                assert_eq!(length, payload_size);
            } else {
                std::ptr::copy_nonoverlapping(iov.iov_base, data_ptr, payload_size);
            }
        }

        let endpoint = endpoint as *mut Endpoint;
        data.shmem.lock_fifo((*endpoint).local_rank.try_into().unwrap(), |fifo| {
            fifo.push(block);
        });
*/

        OPAL_SUCCESS
    })
}

/// Register an error handler.
#[no_mangle]
unsafe extern "C" fn mca_btl_rsm_register_error(
    btl: *mut mca_btl_base_module_t,
    cbfunc: mca_btl_base_module_error_cb_fn_t,
) -> c_int {
    info!("registering error");
    local_data::lock(btl, |data| {
        data.error_cb = cbfunc;
        OPAL_SUCCESS
    })
}
