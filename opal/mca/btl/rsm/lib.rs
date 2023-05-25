use std::os::raw::c_int;
mod opal;
use opal::mca_btl_base_module_t;

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
extern "C" fn mca_btl_rsm_component_register() -> c_int {
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
