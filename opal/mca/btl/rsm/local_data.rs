//! Code for handling private data for the module
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};
use crate::endpoint::Endpoint;
use crate::opal::{
    mca_btl_base_module_error_cb_fn_t,
    mca_btl_base_module_t,
    mca_btl_rsm_t,
};
use crate::shared::{SharedRegionMap, BlockID};
use crate::fifo::FIFO;
use crate::block_store::BlockStore;

/// Data internal to the module.
pub(crate) struct LocalData {
    /// Shared memory for all ranks
    pub(crate) map: Arc<Mutex<SharedRegionMap>>,
    /// Local FIFO
    pub(crate) fifo: FIFO,
    /// Local block store
    pub(crate) block_store: BlockStore,
    /// Pending blocks (local_rank, block_id)
    pub(crate) pending: Vec<(u16, BlockID)>,
    /// Error handler
    pub(crate) error_cb: mca_btl_base_module_error_cb_fn_t,
    /// Endpoints that have access to the shared memory
    pub(crate) endpoints: Vec<*mut Endpoint>,
}

/// Initialize the private module data for the BTL module.
pub(crate) unsafe fn init(
    btl: *mut mca_btl_base_module_t,
    map: Arc<Mutex<SharedRegionMap>>,
    fifo: FIFO,
    block_store: BlockStore,
) {
    let btl = btl as *mut mca_btl_rsm_t;
    let data = Mutex::new(LocalData {
        map,
        fifo,
        block_store,
        pending: vec![],
        error_cb: None,
        endpoints: vec![],
    });
    (*btl).internal = Box::into_raw(Box::new(data)) as *mut _;
}

/// Free the private module data for the BTL module.
pub(crate) unsafe fn free(btl: *mut mca_btl_base_module_t) {
    let btl = btl as *mut mca_btl_rsm_t;
    let data = Box::from_raw((*btl).internal as *mut Mutex<LocalData>);
    // Destory remaining endpoints
    let handle = data.lock().expect("Failed to lock module data");
    for ep in &handle.endpoints {
        let _ = Box::from_raw(*ep);
    }
}

/// Use the module data for the given BTL pointer. The BTL pointer must be
/// valid. LocalData is protected by a wrapping mutex.
pub(crate) unsafe fn lock<F, R>(btl: *mut mca_btl_base_module_t, f: F) -> R
where
    F: FnOnce(&mut LocalData) -> R,
{
    let btl = btl as *mut mca_btl_rsm_t;
    let data = (*btl).internal as *mut Mutex<LocalData>;
    // TODO: This might be better as a try_lock?
    let mut data = (*data).lock().expect("Failed to lock module data");
    f(data.deref_mut())
}
