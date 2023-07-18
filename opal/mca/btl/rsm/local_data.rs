//! Code for handling private data for the module
use crate::block_store::BlockStore;
use crate::endpoint::Endpoint;
use crate::fifo::FIFO;
use crate::opal::{mca_btl_base_module_error_cb_fn_t, mca_btl_base_module_t, mca_btl_rsm_t};
use crate::shared::{BlockID, Descriptor, SharedRegionMap};
use crate::Rank;
use std::cell::RefCell;
use std::ops::DerefMut;
use std::rc::Rc;
use rustc_hash::FxHashMap;

/// Data internal to the module.
pub(crate) struct LocalData {
    /// Shared memory for all ranks
    pub(crate) map: Rc<RefCell<SharedRegionMap>>,
    /// Local FIFO
    pub(crate) fifo: FIFO,
    /// Local block store
    pub(crate) block_store: BlockStore,
    /// Pending blocks (endpoint, block_id)
    ///
    /// TODO: What if the endpoint get's freed before the pending entry is cleared?
    pub(crate) pending: Vec<(usize, BlockID)>,
    /// Error handler
    pub(crate) error_cb: mca_btl_base_module_error_cb_fn_t,
    /// Endpoints that have access to the shared memory
    pub(crate) endpoints: Vec<Option<Endpoint>>,
    /// Descriptor list
    descriptors: FxHashMap<(Rank, BlockID), *mut Descriptor>,
}

impl LocalData {
    /// Create a new descriptor for the rank and block ID and return the pointer.
    #[inline]
    pub(crate) fn new_descriptor(&mut self, rank: Rank, block_id: BlockID) -> *mut Descriptor {
        let des = self.map.borrow_mut().descriptor(rank, block_id);
        let des = Box::new(des);
        let des_ptr = Box::into_raw(des);
        self.descriptors.insert((rank, block_id), des_ptr);
        des_ptr
    }

    /// Find the descriptor with the rank and block ID (used on return of a
    /// descriptor from another process).
    pub(crate) fn find_descriptor(&self, rank: Rank, block_id: BlockID) -> Option<*mut Descriptor> {
        self.descriptors
            .get(&(rank, block_id))
            .map(|des| *des)
    }

    /// Free a descriptor allocated above.
    #[inline]
    pub(crate) unsafe fn free_descriptor(&mut self, des: *mut Descriptor) {
        let rank = (*des).rank;
        let block_id = (*des).block_id;
        let _ = self.descriptors.remove(&(rank, block_id));
        let _ = Box::from_raw(des);
    }

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
    let data = LocalData {
        map,
        fifo,
        block_store,
        pending: vec![],
        error_cb: None,
        endpoints: vec![],
        descriptors: FxHashMap::default(),
    };
    (*btl).internal = Box::into_raw(Box::new(data)) as *mut _;
}

/// Free the private module data for the BTL module.
pub(crate) unsafe fn free(btl: *mut mca_btl_base_module_t) {
    let btl = btl as *mut mca_btl_rsm_t;
    let _ = Box::from_raw((*btl).internal as *mut LocalData);
}

/// Use the module data for the given BTL pointer. The BTL pointer must be
/// valid. LocalData is protected by a wrapping mutex.
pub(crate) unsafe fn lock<F, R>(btl: *mut mca_btl_base_module_t, f: F) -> R
where
    F: FnOnce(&mut LocalData) -> R,
{
    let btl = btl as *mut mca_btl_rsm_t;
    let data = (*btl).internal as *mut LocalData;
    f(data.as_mut().unwrap())
}
