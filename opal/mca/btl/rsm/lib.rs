/// The component type is defined in C
use std::os::raw::c_int;
#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(improper_ctypes)]
mod opal;
use opal::{mca_btl_base_module_t, mca_base_component_var_register};
mod module;

#[no_mangle]
extern "C" fn mca_btl_rsm_component_progress() -> c_int {
    0
}

#[no_mangle]
extern "C" fn mca_btl_rsm_component_open() -> c_int {
    0
}

#[no_mangle]
extern "C" fn mca_btl_rsm_component_close() -> c_int {
    0
}

#[no_mangle]
extern "C" fn mca_btl_rsm_component_register_params() -> c_int {
    0
}

#[no_mangle]
extern "C" fn mca_btl_rsm_component_init(
    num_btls: *mut c_int,
    enable_progress_threads: bool,
    enable_mpi_threads: bool,
) -> *mut *mut mca_btl_base_module_t {
    std::ptr::null_mut()
}
