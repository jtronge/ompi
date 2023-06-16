use std::sync::atomic::AtomicU64;

#[repr(C)]
pub(crate) struct FIFO {
    head: u64,
    tail: AtomicU64,
}
