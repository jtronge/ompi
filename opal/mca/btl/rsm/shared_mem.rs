use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicU8, Ordering};
use shared_memory::{ShmemConf, Shmem};
use crate::{Result, Error};
use crate::opal::mca_btl_base_tag_t;

#[derive(Clone, Debug)]
pub struct SharedMemoryOptions {
    pub backing_directory: String,
    pub num_local_peers: u32,
    pub node_name: String,
    pub node_rank: u16,
    pub euid: u32,
    pub jobid: u32,
}

/// Build the path for the shared memory file from the options.
fn make_path(opts: &SharedMemoryOptions) -> PathBuf {
    let fname = format!("{}-{}-{}", opts.node_name, opts.node_rank, opts.jobid);
    let mut path = PathBuf::new();
    path.push("tmp");
    path.push(fname);
    path
}

/// Get the address of block i.
unsafe fn block_addr(blocks: *mut u8, i: isize) -> *mut u8 {
    assert!(i >= 0);
    let block_size: isize = inner::BLOCK_SIZE.try_into().unwrap();
    blocks.offset(i * block_size)
}

/// Wrapper managing access to the FIFOs in shared memory
#[derive(Clone)]
struct FIFOArray {
    shmem: Arc<Shmem>,
    fifos: *mut inner::FIFO,
}

impl FIFOArray {
    /// Lock the FIFO and pass a mutable reference into the callback.
    pub fn lock_fifo<F, R>(&self, fifo_id: isize, f: F) -> R
    where
        F: FnOnce(&mut inner::FIFO) -> R,
    {
        unsafe {
            let max_peers: isize = inner::MAX_PEERS.try_into().unwrap();
            assert!(fifo_id < max_peers);
            let fifo = self.fifos.offset(fifo_id).as_mut().unwrap();
            while fifo.lock.compare_exchange(0, 1, Ordering::SeqCst,
                                             Ordering::Relaxed).is_err() {}
            let res = f(fifo);
            fifo.lock.store(0, Ordering::SeqCst);
            res
        }
    }
}

/// Shared memory free list wrapper
#[derive(Clone)]
struct FreeList {
    shmem: Arc<Shmem>,
    free_list: *mut inner::FIFO,
}

impl FreeList {
    /// Lock the free list and pass a mutable reference to the callback.
    unsafe fn lock<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce(&mut inner::FIFO) -> R,
    {
        let free_list = unsafe { self.free_list.as_mut().unwrap() };
        while free_list.lock.compare_exchange(0, 1, Ordering::SeqCst,
                                              Ordering::Relaxed).is_err() {}
        let r = f(free_list);
        free_list.lock.store(0, Ordering::SeqCst);
        r
    }
}

/// Main shared memory handle. Use to access FIFOs and blocks in shared memory.
pub struct SharedMemory {
    shmem: Arc<Shmem>,
    fifos: FIFOArray,
    free_list: FreeList,
    blocks: *mut u8,
}

impl SharedMemory {
    /// Create and initialize the shared memory.
    pub unsafe fn create(opts: SharedMemoryOptions) -> Result<SharedMemory> {
        let path = make_path(&opts);
        let shmem = ShmemConf::new()
            .size(inner::SHMEM_SIZE)
            .create()
            .unwrap();
        let ptr = shmem.as_ptr();

        let fifos = ptr as *mut inner::FIFO;
        std::ptr::write_bytes(ptr, 0, inner::SHMEM_SIZE);

        // Initialize the fifo
        let mut fifo = ptr as *mut inner::FIFO;
        for _ in 0..inner::MAX_PEERS {
            (*fifo).lock.store(0, Ordering::Relaxed);
            (*fifo).head = -1;
            (*fifo).tail = -1;
            (*fifo).count = 0;
            fifo = fifo.offset(1);
        }

        let free_list = fifo;
        // Initialize the free list
        (*fifo).lock.store(0, Ordering::Relaxed);
        (*fifo).head = 0;
        (*fifo).tail = (inner::MAX_BLOCKS - 1).try_into().unwrap();
        (*fifo).count = inner::MAX_BLOCKS;
        let block_offset: isize = inner::BLOCK_SIZE.try_into().unwrap();
        let mut ptr = fifo.offset(1) as *mut u8;
        let blocks = ptr;
        for i in 0..inner::MAX_BLOCKS {
            let block = ptr as *mut inner::BlockHeader;
            (*block).next = if (i + 1) < inner::MAX_BLOCKS { (i + 1).try_into().unwrap() } else { -1 };
            (*block).len = 0;
            ptr = ptr.offset(block_offset);
        }

        // Create mutexes for the fifos array and the free list
        let shmem = Arc::new(shmem);
        let fifos = FIFOArray {
            shmem: Arc::clone(&shmem),
            fifos,
        };
        let free_list = FreeList {
            shmem: Arc::clone(&shmem),
            free_list,
        };
        Ok(SharedMemory {
            shmem,
            fifos,
            free_list,
            blocks,
        })
    }

    /// Allocate a block from the free store.
    pub fn alloc(&mut self, len: usize) -> Result<Block> {
        unsafe {
            let blocks = self.blocks;
            let free_list_clone = self.free_list.clone();
            self.free_list.lock(|free_list| {
                if free_list.head < 0 {
                    Err(Error::OOM)
                } else {
                    unsafe {
                        let block_id = free_list.head;
                        let ptr = block_addr(blocks, block_id);

                        let hdr = ptr as *mut inner::BlockHeader;
                        let next = (*hdr).next;
                        (*hdr).next = -1;
                        (*hdr).len = len;

                        free_list.head = next;
                        if next < 0 {
                            free_list.tail = -1;
                        }
                        free_list.count -= 1;

                        Ok(Block {
                            block: ptr,
                            block_id,
                            free_list: free_list_clone,
                        })
                    }
                }
            })
        }
    }

    /// Lock the FIFO with ID fifo_id and pass it into the callback.
    pub fn lock_fifo<F, R>(&self, fifo_id: isize, f: F) -> R
    where
        F: FnOnce(&mut FIFO) -> R,
    {
        let shmem = Arc::clone(&self.shmem);
        let blocks = self.blocks;
        let free_list = self.free_list.free_list;
        self.fifos
            .lock_fifo(fifo_id, |fifo| {
                let mut fifo = FIFO {
                    shmem,
                    fifo: fifo,
                    free_list,
                    blocks,
                };
                f(&mut fifo)
            })
    }
}

/// A FIFO corresponding to a particular process
pub struct FIFO<'a> {
    shmem: Arc<Shmem>,
    fifo: &'a mut inner::FIFO,
    /// Free list pointer required for blocks
    free_list: *mut inner::FIFO,
    blocks: *mut u8,
}

impl<'a> FIFO<'a> {
    pub fn push(&mut self, block: Block) {
        unsafe {
            let ptr = block_addr(self.blocks, block.block_id);
            let hdr = ptr as *mut inner::BlockHeader;
            (*hdr).next = -1;
            if self.fifo.tail == -1 {
                self.fifo.head = block.block_id;
            } else {
                let old_tail_hdr = block_addr(self.blocks, self.fifo.tail) as *mut inner::BlockHeader;
                (*old_tail_hdr).next = block.block_id;
            }
            self.fifo.tail = block.block_id;
            self.fifo.count += 1;
            // We don't want drop to be called
            std::mem::forget(block);
        }
    }

    pub fn pop(&mut self) -> Option<Block> {
        if self.fifo.count > 0 {
            unsafe {
                let old_head = self.fifo.head;
                let ptr = block_addr(self.blocks, old_head);
                let old_hdr = ptr as *mut inner::BlockHeader;
                let next = (*old_hdr).next;
                if next == -1 {
                    // Last block
                    assert_eq!(self.fifo.head, self.fifo.tail);
                    self.fifo.tail = -1;
                }
                self.fifo.head = next;
                Some(Block {
                    block: ptr,
                    block_id: old_head,
                    free_list: FreeList {
                        shmem: Arc::clone(&self.shmem),
                        free_list: self.free_list,
                    },
                })
            }
        } else {
            None
        }
    }

    pub fn count(&self) -> usize {
        self.fifo.count
    }
}

/// A block of shared memory.
pub struct Block {
    block: *mut u8,
    block_id: isize,
    free_list: FreeList,
}

impl Block {
    /// Return a mutable pointer to the block body.
    pub fn as_mut(&mut self) -> *mut u8 {
        unsafe {
            let hdr = self.block as *mut inner::BlockHeader;
            // It doesn't make much sense that offset() is marked unsafe, it
            // should only be the dereferencing of a pointer that's unsafe
            hdr.offset(1) as *mut _
        }
    }

    /// Return an immutable pointer to the block body.
    pub fn as_ptr(&self) -> *const u8 {
        unsafe {
            let hdr = self.block as *const inner::BlockHeader;
            hdr.offset(1) as *const _
        }
    }

    /// Return the length of the block body.
    pub fn len(&self) -> usize {
        unsafe {
            let hdr = self.block as *mut inner::BlockHeader;
            (*hdr).len
        }
    }

    /// Return the tag.
    pub fn tag(&self) -> mca_btl_base_tag_t {
        unsafe {
            let hdr = self.block as *mut inner::BlockHeader;
            (*hdr).tag
        }
    }

    /// Set the tag for a block.
    pub fn set_tag(&mut self, tag: mca_btl_base_tag_t) {
        unsafe {
            let hdr = self.block as *mut inner::BlockHeader;
            (*hdr).tag = tag;
        }
    }

    /// Local rank of the source.
    pub fn src(&self) -> u16 {
        unsafe {
            let hdr = self.block as *mut inner::BlockHeader;
            (*hdr).src
        }
    }

    /// Set the local rank of the source.
    pub fn set_src(&mut self, src: u16) {
        unsafe {
            let hdr = self.block as *mut inner::BlockHeader;
            (*hdr).src = src;
        }
    }
}

impl Drop for Block {
    fn drop(&mut self) {
        unsafe {
            let hdr = self.block as *mut inner::BlockHeader;
            (*hdr).next = -1;
            (*hdr).tag = 0;
            (*hdr).len = 0;
            // TODO: Lock the free list and get rid o
        }
    }
}

mod inner {
    use std::sync::atomic::AtomicU8;
    use std::mem::size_of;
    use crate::opal::mca_btl_base_tag_t;

    #[repr(C)]
    pub struct BlockHeader {
        // TODO: Need tag
        pub src: u16,
        pub tag: mca_btl_base_tag_t,
        pub next: isize,
        pub len: usize,
    }

    #[repr(C)]
    pub struct FIFO {
        pub lock: AtomicU8,
        pub head: isize,
        pub tail: isize,
        pub count: usize,
    }

    pub const MAX_PEERS: usize = 512;
    pub const MAX_BLOCKS: usize = 1024;
    pub const BLOCK_BODY: usize = 1024;
    pub const BLOCK_SIZE: usize = size_of::<BlockHeader>() + BLOCK_BODY;
    /// SHMEM_SIZE includes the FIFOs for each process, plus a free list on the
    /// end, and MAX_BLOCKS blocks
    pub const SHMEM_SIZE: usize = (MAX_PEERS + 1) * size_of::<FIFO>()
                                  + MAX_BLOCKS * BLOCK_SIZE;
}
