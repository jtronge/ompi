use std::sync::atomic::AtomicI32;
use std::path::PathBuf;
use shared_memory::{ShmemConf, Shmem};

const BLOCK_COUNT: usize = 32;
const BLOCK_SIZE: usize = 8129;

#[repr(C)]
struct FIFOHeader {
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

const SHARED_SIZE: usize = std::mem::size_of::<FIFOHeader>() + BLOCK_COUNT * std::mem::size_of::<Block>();

pub struct SharedMemory {
    shmem: Shmem,
}

fn make_path(pid: i32, node: u32, local_rank: u16) -> PathBuf {
    let fname = format!("{}-{}-{}", pid, node, local_rank);
    let mut path = PathBuf::new();
    path.push("tmp");
    path.push(fname);
    path
}

impl SharedMemory {
    pub fn create(pid: i32, node: u32, local_rank: u16) -> SharedMemory {
        let path = make_path(pid, node, local_rank);
        let shmem = ShmemConf::new()
            .size(SHARED_SIZE)
            .create()
            .unwrap();
        // First zero everything
        unsafe {
            // This is slow
            let mut ptr = shmem.as_ptr();
            for i in 0..SHARED_SIZE {
                *ptr = 0;
                ptr = ptr.offset(1);
            }
        }
        let mut shared = SharedMemory { shmem };
        let block_count: i32 = BLOCK_COUNT.try_into().unwrap();
        // Initialize the header
        let header = shared.header();
        header.head = -1;
        header.tail = -1;
        header.free_head = 0;
        header.free_tail = block_count - 1;
        // Initialize the block links
        let blocks = shared.blocks();
        for (i, block) in blocks.iter_mut().enumerate() {
            let i: i32 = i.try_into().unwrap();
            let prev = if i == 0 {
                -1
            } else {
                i - 1
            };
            let next = if i == block_count - 1 {
                -1
            } else {
                i + 1
            };
            block.next = next;
            block.prev = prev;
        }
        shared
    }

    pub fn open(pid: i32, node: u32, local_rank: u16) -> SharedMemory {
        let path = make_path(pid, node, local_rank);
        let shmem = ShmemConf::new()
            .size(SHARED_SIZE)
            .open()
            .unwrap();
        SharedMemory {
            shmem,
        }
    }

    pub fn header(&mut self) -> &mut FIFOHeader {
        unsafe {
            (self.shmem.as_ptr() as *mut FIFOHeader).as_mut().unwrap()
        }
    }

    pub fn blocks(&mut self) -> &mut [Block] {
        unsafe {
            let ptr = self.shmem
                .as_ptr()
                .offset(std::mem::size_of::<FIFOHeader>().try_into().unwrap());
            std::slice::from_raw_parts_mut(
                ptr as *mut Block,
                BLOCK_COUNT,
            )
        }
    }
}
