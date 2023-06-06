use std::sync::Mutex;
use crate::module::PendingBlock;
use crate::shared_mem::SharedMemory;

/// Global for open shared memory
pub(crate) static mut SHMEM: Option<SharedMemory> = None;
/// Pending blocks/segments
pub(crate) static mut PENDING: Option<Mutex<Vec<PendingBlock>>> = None;
