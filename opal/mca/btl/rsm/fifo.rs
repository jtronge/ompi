use std::sync::atomic::AtomicI32;

const BLOCK_SIZE: usize = 8129;

#[repr(C)]
struct FIFOHeader {
    lock: AtomicI32,
    head: i32,
    tail: i32,
    free_head: i32,
    free_tail: i32,
}

#[repr(C)]
struct Block {
    next: i32,
    prev: i32,
    data: [u8; BLOCK_SIZE],
}
