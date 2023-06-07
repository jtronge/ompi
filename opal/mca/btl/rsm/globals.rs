use std::sync::Mutex;
use crate::module::{PendingBlock, Endpoint};
use crate::shared_mem::SharedMemory;
use crate::opal::mca_btl_base_module_error_cb_fn_t;

/// Global for open shared memory
pub(crate) static mut SHMEM: Option<SharedMemory> = None;
/// Pending blocks/segments
pub(crate) static mut PENDING: Option<Mutex<Vec<PendingBlock>>> = None;
/// Global error callback
pub(crate) static mut ERROR_CB: mca_btl_base_module_error_cb_fn_t = None;
/// Endpoints
pub(crate) static mut ENDPOINTS: Option<Mutex<Vec<*mut Endpoint>>> = None;
