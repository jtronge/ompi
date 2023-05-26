/// The component type is defined in C
use std::os::raw::c_int;
#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(improper_ctypes)]
mod opal;
use opal::{
    mca_btl_base_module_t,
    mca_base_component_var_register,
    calloc,
};
mod module;
mod shmem;
mod fifo;
mod modex;

extern "C" {
    pub static mut mca_btl_rsm: mca_btl_base_module_t;
}

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

    // TODO: Create the segment
    // TODO: Create the FIFO

    *btls = &mut mca_btl_rsm;
    btls
}

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
    // Ignoring params for now
    0
}
