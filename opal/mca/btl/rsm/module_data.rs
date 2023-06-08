//! Code for handling private data for the module
use std::sync::Mutex;
use std::ops::DerefMut;
use crate::module::{Endpoint, PendingBlock};
use crate::opal::{
    mca_btl_base_module_error_cb_fn_t,
    mca_btl_base_module_t,
    mca_btl_rsm_t,
};
use crate::shared_mem::SharedMemory;

/// Data internal to the module.
pub(crate) struct ModuleData {
    /// Shared memory store
    pub(crate) shmem: SharedMemory,
    /// Pending blocks
    pub(crate) pending: Vec<PendingBlock>,
    /// Error handler
    pub(crate) error_cb: mca_btl_base_module_error_cb_fn_t,
    /// Endpoints that have access to the shared memory
    pub(crate) endpoints: Vec<*mut Endpoint>,
}

/// Initialize the private module data for the BTL module.
pub(crate) unsafe fn init(btl: *mut mca_btl_base_module_t, shmem: SharedMemory) {
    let btl = btl as *mut mca_btl_rsm_t;
    let data = Mutex::new(ModuleData {
        shmem,
        pending: vec![],
        error_cb: None,
        endpoints: vec![],
    });
    (*btl).internal = Box::into_raw(Box::new(data)) as *mut _;
}

/// Free the private module data for the BTL module.
pub(crate) unsafe fn free(btl: *mut mca_btl_base_module_t) {
    let btl = btl as *mut mca_btl_rsm_t;
    let data = Box::from_raw((*btl).internal as *mut Mutex<ModuleData>);
    // Destory remaining endpoints
    let handle = data.lock().expect("Failed to lock module data");
    for ep in &handle.endpoints {
        let _ = Box::from_raw(*ep);
    }
}

/// Use the module data for the given BTL pointer. The BTL pointer must be
/// valid. ModuleData is protected by a wrapping mutex.
pub(crate) unsafe fn lock<F, R>(btl: *mut mca_btl_base_module_t, f: F) -> R
where
    F: FnOnce(&mut ModuleData) -> R,
{
    let btl = btl as *mut mca_btl_rsm_t;
    let data = (*btl).internal as *mut Mutex<ModuleData>;
    // TODO: This might be better as a try_lock?
    let mut data = (*data).lock().expect("Failed to lock module data");
    f(data.deref_mut())
}
