//! Code for handling private data for the module
use std::sync::Mutex;
use std::ops::DerefMut;
use std::os::raw::{c_int, c_void};
use std::mem::MaybeUninit;
use shared_memory::Shmem;
use crate::endpoint::Endpoint;
use crate::opal::{
    mca_btl_base_module_error_cb_fn_t,
    mca_btl_base_module_t,
    mca_btl_base_tag_t,
    mca_btl_rsm_t,
    opal_convertor_t,
};
use crate::block::{BLOCK_SIZE, Block};
use crate::fifo::FIFO;

pub const MAX_BLOCKS: usize = 128;
pub const SHARED_MEM_SIZE: usize = std::mem::size_of::<FIFO>() + MAX_BLOCKS * std::mem::size_of::<Block>();

pub(crate) struct Descriptor;

/// Data internal to the module.
pub(crate) struct LocalData {
    /// Shared memory store
    pub(crate) shmem: Shmem,
    /// Pending blocks (local_rank, block_id)
    pub(crate) pending: Vec<(u16, isize)>,
    /// Error handler
    pub(crate) error_cb: mca_btl_base_module_error_cb_fn_t,
    /// Endpoints that have access to the shared memory
    pub(crate) endpoints: Vec<*mut Endpoint>,
}

impl LocalData {
    /// Allocate a new block from the free store.
    pub(crate) fn alloc(&mut self) -> isize {
        -1
    }

    /// Use the block with the given block_id.
    pub(crate) fn use_block<F, R>(&mut self, block_id: isize, f: F) -> R
    where
        F: FnOnce(&mut Block) -> R,
    {
        let data = unsafe { MaybeUninit::<[u8; BLOCK_SIZE]>::uninit().assume_init() };
        let mut block = Block {
            next: 0,
            tag: 0,
            message_trigger: 0,
            complete: false,
            len: 0,
            data,
        };
        f(&mut block)
    }

    /// Create and return a descriptor for a block_id.
    pub(crate) fn descriptor(&self, block_id: isize) -> Descriptor {
        Descriptor
    }

    /// Pop an incoming block off the FIFO.
    pub(crate) fn pop(&mut self) -> Option<(u16, isize)> {
        None
    }
}

/// Initialize the private module data for the BTL module.
pub(crate) unsafe fn init(btl: *mut mca_btl_base_module_t, shmem: Shmem) {
    let btl = btl as *mut mca_btl_rsm_t;
    let data = Mutex::new(LocalData {
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
