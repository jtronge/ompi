//! Code for handling private data for the module
use crate::block_store::BlockStore;
use crate::endpoint::Endpoint;
use crate::fifo::FIFO;
use crate::opal::{mca_btl_base_module_error_cb_fn_t, mca_btl_base_module_t, mca_btl_rsm_t};
use crate::shared::SharedRegionMap;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Mutex;
use log::info;

/// Data internal to the module.
pub(crate) struct LocalData {
    /// Shared memory for all ranks
    pub(crate) map: Rc<RefCell<SharedRegionMap>>,
    /// Local FIFO
    pub(crate) fifo: FIFO,
    /// Local block store
    pub(crate) block_store: BlockStore,
    /// Error handler
    pub(crate) error_cb: mca_btl_base_module_error_cb_fn_t,
    /// Endpoints that have access to the shared memory
    pub(crate) endpoints: Vec<Option<Endpoint>>,
}

impl LocalData {
    /// Add an endpoint
    pub(crate) fn add_endpoint(&mut self, endpoint: Endpoint) -> usize {
        self.endpoints.push(Some(endpoint));
        self.endpoints.len() - 1
    }

    /// Delete endpoint
    pub(crate) fn del_endpoint(&mut self, endpoint_idx: usize) {
        let _ = self.endpoints[endpoint_idx].take();
    }
}

/// Initialize the private module data for the BTL module.
pub(crate) unsafe fn init(
    btl: *mut mca_btl_base_module_t,
    map: Rc<RefCell<SharedRegionMap>>,
    fifo: FIFO,
    block_store: BlockStore,
) {
    let btl = btl as *mut mca_btl_rsm_t;
    let data = Mutex::new(LocalData {
        map,
        fifo,
        block_store,
        error_cb: None,
        endpoints: vec![],
    });
    (*btl).internal = Box::into_raw(Box::new(data)) as *mut _;
}

/// Free the private module data for the BTL module.
pub(crate) unsafe fn free(btl: *mut mca_btl_base_module_t) {
    let btl = btl as *mut mca_btl_rsm_t;
    let _ = Box::from_raw((*btl).internal as *mut Mutex<LocalData>);
}

/// Use the module data for the given BTL pointer. The BTL pointer must be
/// valid. LocalData is protected by a wrapping mutex.
pub(crate) unsafe fn lock<F, R>(btl: *mut mca_btl_base_module_t, f: F) -> R
where
    F: FnOnce(&mut LocalData) -> R,
{
    let btl = btl as *mut mca_btl_rsm_t;
    info!("Before mutex...");
    let data = (*btl).internal as *mut Mutex<LocalData>;
    let mut data = (*data).lock().unwrap();
    info!("After lock");
    f(&mut data)
}
