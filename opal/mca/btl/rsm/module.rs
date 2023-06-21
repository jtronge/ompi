use std::os::raw::{c_int, c_void};
use std::sync::Arc;
use std::cell::RefCell;
use std::sync::atomic::Ordering;
use log::info;
use crate::opal::{
    mca_btl_base_descriptor_t,
    mca_btl_base_endpoint_t,
    mca_btl_base_module_error_cb_fn_t,
    mca_btl_base_module_t,
    mca_btl_base_tag_t,
    opal_bitmap_set_bit,
    opal_bitmap_t,
    opal_convertor_t,
    opal_proc_local_get,
    opal_proc_t,
    opal_proc_on_local_node_rs,
    OPAL_SUCCESS,
    OPAL_ERR_OUT_OF_RESOURCE,
};
use crate::modex::{self, Key};
use crate::Rank;
use crate::proc_info;
use crate::local_data;
use crate::endpoint::Endpoint;
use crate::shared::{SharedRegionHandle, Descriptor, make_path, FIFO_FREE};

#[no_mangle]
unsafe extern "C" fn mca_btl_rsm_add_procs(
    btl: *mut mca_btl_base_module_t,
    nprocs: usize,
    procs: *mut *mut opal_proc_t,
    peers: *mut *mut mca_btl_base_endpoint_t,
    reachability: *mut opal_bitmap_t,
) -> c_int {
    info!("adding procs");
    if reachability.is_null() {
        return 0;
    }
    let my_proc = opal_proc_local_get();
    if my_proc.is_null() {
        return OPAL_ERR_OUT_OF_RESOURCE;
    }
    local_data::lock(btl, |data| {
        let mut rc = 0;
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
            let local_rank: Rank = local_rank.into();

            // Attach to the memory region
            let path = make_path(&proc_info::node_name(), local_rank);
            let region = match SharedRegionHandle::attach(path) {
                Ok(reg) => reg,
                // TODO: Propagate this error
                Err(_) => continue,
            };
            data.map.lock().unwrap().regions.insert(local_rank, RefCell::new(region));

            // Create the endpoint
            let endpoint = match Endpoint::new(Arc::clone(&data.map), local_rank) {
                Ok(ep) => ep,
                // TODO: Propagate this error
                Err(_) => continue,
            };
            let endpoint = Box::new(endpoint);
            let endpoint_ptr = Box::into_raw(endpoint);
            *peers.offset(proc) = endpoint_ptr as *mut _;
            data.endpoints.push(endpoint_ptr);
        }
        rc
    })
}

#[no_mangle]
unsafe extern "C" fn mca_btl_rsm_del_procs(
    btl: *mut mca_btl_base_module_t,
    nprocs: usize,
    _procs: *mut *mut opal_proc_t,
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
                // Remove the region from the store
                let _ = data.map.lock().unwrap().regions.remove(&(*ep).rank);

                // Convert back to a Box and thus free it
                let _ = Box::from_raw(ep);
                *peer = std::ptr::null_mut();
            }
        }
        OPAL_SUCCESS
    })
}

#[no_mangle]
unsafe extern "C" fn mca_btl_rsm_finalize(
    btl: *mut mca_btl_base_module_t,
) -> c_int {
    info!("running finalize");
    local_data::free(btl);
    OPAL_SUCCESS
}

/// Allocate a new descriptor and return it.
#[no_mangle]
unsafe extern "C" fn mca_btl_rsm_alloc(
    btl: *mut mca_btl_base_module_t,
    _endpoint: *mut mca_btl_base_endpoint_t,
    _order: u8,
    size: usize,
    _flags: u32,
) -> *mut mca_btl_base_descriptor_t {
    info!("allocating a descriptor of size {}", size);
    local_data::lock(btl, |data| {
        // TODO: Set length
        let block_id = match data.block_store.alloc() {
            Some(id) => id,
            None => return std::ptr::null_mut(),
        };
        let desc = data
            .map
            .lock()
            .unwrap()
            .descriptor(proc_info::local_rank(), block_id);
        Box::into_raw(Box::new(desc)) as *mut _
    })
}

/// Free a descriptor.
#[no_mangle]
unsafe extern "C" fn mca_btl_rsm_free(
    btl: *mut mca_btl_base_module_t,
    des: *mut mca_btl_base_descriptor_t,
) -> c_int {
    info!("freeing descriptor");
    let des = Box::from_raw(des as *mut Descriptor);
    local_data::lock(btl, |data| {
        if des.rank == proc_info::local_rank() {
            // Only release block if it's owned by this node
            data.block_store.free(des.block_id);
        }
        // TODO: In what case would this block come from a different node's
        // shared memory?
        OPAL_SUCCESS
    })
}

/// Packed data into shared memory.
#[no_mangle]
unsafe extern "C" fn mca_btl_rsm_prepare_src(
    btl: *mut mca_btl_base_module_t,
    _endpoint: *mut mca_btl_base_endpoint_t,
    convertor: *mut opal_convertor_t,
    _order: u8,
    reserve: usize,
    size: *mut usize,
    _flags: u32,
) -> *mut mca_btl_base_descriptor_t {
    info!("calling prepare_src");
    local_data::lock(btl, |data| {
        let block_id = match data.block_store.alloc() {
            Some(id) => id,
            None => return std::ptr::null_mut(),
        };
        let rc = data.map.lock().unwrap().region_mut(proc_info::local_rank(), |region| {
            let block_idx: usize = block_id.try_into().unwrap();
            region.blocks[block_idx].prepare_fill(convertor, reserve, size)
        });
        if rc < 0 {
            return std::ptr::null_mut();
        }
        // TODO: Set order and flags
        let desc = data.map.lock().unwrap().descriptor(proc_info::local_rank(), block_id);
        let desc = Box::new(desc);
        Box::into_raw(desc) as *mut _
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
        let block_idx: usize = block_id.try_into().unwrap();
        data.map.lock().unwrap().region_mut(proc_info::local_rank(), |region| {
            region.blocks[block_idx].tag = tag;
        });
        // The original SM attempts a write into the peer's fifo, here it
        // either writes or fails altogether
        (*endpoint).fifo.push(proc_info::local_rank(), block_id).unwrap();
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
    _order: u8,
    _flags: u32,
    tag: mca_btl_base_tag_t,
    descriptor: *mut *mut mca_btl_base_descriptor_t,
) -> c_int {
    info!("calling sendi");
    local_data::lock(btl, |data| {
        // Check pending, return early if there are some
        if data.pending.len() > 0 {
            return OPAL_ERR_OUT_OF_RESOURCE;
        }

        // Alloc block and set output descriptor
        let block_id = match data.block_store.alloc() {
            Some(id) => id,
            None => return OPAL_ERR_OUT_OF_RESOURCE,
        };
        let endpoint = endpoint as *mut Endpoint;

        // Set the block data
        data.map.lock().unwrap().region_mut(proc_info::local_rank(), |region| {
            let block_idx: usize = block_id.try_into().unwrap();
            let block = &mut region.blocks[block_idx];
            block.next.store(FIFO_FREE, Ordering::Relaxed);
            block.tag = tag;
            block.complete = false;
            block.fill(convertor, header, header_size, payload_size);
        });

        // Push the block on to the endpoint's FIFO
        (*endpoint).fifo.push(proc_info::local_rank(), block_id).unwrap();

        // Set output descriptor
        let desc = Box::new(
            data
                .map
                .lock()
                .unwrap()
                .descriptor(proc_info::local_rank(), block_id)
        );
        *descriptor = Box::into_raw(desc) as *mut _;

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
