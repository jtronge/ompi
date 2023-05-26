use std::os::raw::{c_int, c_void};
use crate::opal::{
    mca_btl_base_descriptor_t,
    mca_btl_base_endpoint_t,
    mca_btl_base_module_error_cb_fn_t,
    mca_btl_base_module_t,
    mca_btl_base_tag_t,
    opal_bitmap_t,
    opal_convertor_t,
    opal_proc_t,
};

#[no_mangle]
extern "C" fn mca_btl_rsm_add_procs(
    btl: *mut mca_btl_base_module_t,
    nprocs: usize,
    procs: *mut *mut opal_proc_t,
    peers: *mut *mut mca_btl_base_endpoint_t,
    reachability: *mut opal_bitmap_t,
) -> c_int {
    0
}

#[no_mangle]
extern "C" fn mca_btl_rsm_del_procs(
    btl: *mut mca_btl_base_module_t,
    nprocs: usize,
    procs: *mut *mut opal_proc_t,
    peers: *mut *mut mca_btl_base_endpoint_t,
) -> c_int {
    0
}

#[no_mangle]
extern "C" fn mca_btl_rsm_finalize(btl: *mut mca_btl_base_module_t) -> c_int {
    0
}

#[no_mangle]
extern "C" fn mca_btl_rsm_alloc(
    btl: *mut mca_btl_base_module_t,
    endpoc_int: *mut mca_btl_base_endpoint_t,
    order: u8,
    size: usize,
    flags: u32,
) -> *mut mca_btl_base_descriptor_t {
    std::ptr::null_mut()
}

#[no_mangle]
extern "C" fn mca_btl_rsm_free(
    btl: *mut mca_btl_base_module_t,
    des: *mut mca_btl_base_descriptor_t,
) -> c_int {
    0
}

#[no_mangle]
extern "C" fn mca_btl_rsm_prepare_src(
    btl: *mut mca_btl_base_module_t,
    endpoc_int: *mut mca_btl_base_endpoint_t,
    convertor: *mut opal_convertor_t,
    order: u8,
    reserve: usize,
    size: *mut usize,
    flags: u32,
) -> *mut mca_btl_base_descriptor_t {
    std::ptr::null_mut()
}

#[no_mangle]
extern "C" fn mca_btl_rsm_send(
    btl: *mut mca_btl_base_module_t,
    endpoc_int: *mut mca_btl_base_endpoint_t,
    descriptor: *mut mca_btl_base_descriptor_t,
    tag: mca_btl_base_tag_t,
) -> c_int {
    0
}

#[no_mangle]
extern "C" fn mca_btl_rsm_sendi(
    btl: *mut mca_btl_base_module_t,
    endpoc_int: *mut mca_btl_base_endpoint_t,
    convertor: *mut opal_convertor_t,
    header: *mut c_void,
    header_size: usize,
    payload_size: usize,
    order: u8,
    flags: u32,
    tag: mca_btl_base_tag_t,
    descriptor: *mut *mut mca_btl_base_descriptor_t,
) -> c_int {
    0
}

#[no_mangle]
extern "C" fn mca_btl_rsm_register_error(
    btl: *mut mca_btl_base_module_t,
    cbfunc: mca_btl_base_module_error_cb_fn_t,
) -> c_int  {
    0
}
