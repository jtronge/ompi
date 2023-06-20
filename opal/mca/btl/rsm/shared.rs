//! Shared memory management code.
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicI64, Ordering};
use std::os::raw::{c_void, c_int};
use shared_memory::{ShmemConf, Shmem};
use crate::{Result, Error, Rank};
use std::cell::RefCell;
use crate::opal::{
    mca_btl_base_tag_t,
    iovec,
    opal_convertor_t,
    opal_convertor_get_current_pointer_rs,
    opal_convertor_need_buffers_rs,
    opal_convertor_pack,
};
use crate::local_data::Descriptor;

/// Create a path for a shared memory region.
pub fn make_path(node_name: &str, local_rank: Rank) -> PathBuf {
    let fname = format!("{}-{}.shmem", node_name, local_rank);
    let mut path = PathBuf::new();
    path.push("/dev/shm");
    path.push(fname);
    path
}

/// Data structure holding all shared memory regions for each reachable process.
pub(crate) struct SharedMemoryStore {
    pub regions: HashMap<Rank, RefCell<SharedMemoryRegion>>,
}

impl SharedMemoryStore {
    /// Return a descriptor for a block.
    pub fn descriptor(&self, rank: Rank, block_id: BlockID) -> Descriptor {
        Descriptor
    }
}

/// Block list handle
pub(crate) struct BlockList {
    blocks: *mut Block,
}

impl BlockList {
    /// Get a slice for all the blocks
    pub fn get(&mut self) -> &mut [Block] {
        unsafe {
            std::slice::from_raw_parts_mut(self.blocks, MAX_BLOCKS)
        }
    }
}

pub(crate) struct FIFOHeaderHandle {
    header: *mut FIFOHeader,
}

impl FIFOHeaderHandle {
    /// Get a reference for the header
    pub fn get(&mut self) -> &mut FIFOHeader {
        unsafe {
            self.header.as_mut().unwrap()
        }
    }
}

/// Shared memory wrapper.
pub(crate) struct SharedMemoryRegion {
    /// Shared memory
    shmem: Shmem,
    /// Header handle
    pub fifo: *mut FIFOHeader,
    /// Block list handle
    pub blocks: *mut Block,
}

impl SharedMemoryRegion {
    /// Create a new shared memory path at a given location.
    pub fn create<P: AsRef<Path>>(path: P) -> Result<SharedMemoryRegion> {
        unsafe {
            // SAFETY: The initialization below is safe assuming that the
            // shared memory allocates a region of SHARED_MEM_SIZE. Alignment
            // is checked by comparing the pointer with the value returned by
            // align_of().
            let shmem = ShmemConf::new()
                .size(SHARED_MEM_SIZE)
                .flink(path)
                .create();
            let shmem = if let Ok(shmem) = shmem {
                shmem
            } else {
                return Err(Error::SharedMemoryFailure);
            };

            // Initialize everything
            let ptr = shmem.as_ptr();
            std::ptr::write_bytes(ptr, 0, SHARED_MEM_SIZE);
            let fifo = ptr as *mut FIFOHeader;
            (*fifo).head = FIFO_FREE;
            (*fifo).tail.store(FIFO_FREE, Ordering::Relaxed);
            let blocks = fifo.offset(1) as *mut Block;
            let mut tmp = blocks;
                // Initialize the complete field just in case
            for _ in 0..MAX_BLOCKS {
                (*tmp).complete = false;
                tmp = tmp.offset(1);
            }

            Ok(SharedMemoryRegion {
                shmem,
                fifo,
                blocks,
            })
        }
    }

    /// Attach to an existing shared memory path.
    pub fn attach<P: AsRef<Path>>(path: P) -> Result<SharedMemoryRegion> {
        let shmem = ShmemConf::new()
            .size(SHARED_MEM_SIZE)
            .flink(path)
            .open();
        let shmem = if let Ok(shmem) = shmem {
            shmem
        } else {
            return Err(Error::SharedMemoryFailure);
        };

        let fifo = shmem.as_ptr() as *mut FIFOHeader;
        // I fail to understand why the offset() method is declared unsafe
        let blocks = unsafe { fifo.offset(1) } as *mut Block;

        Ok(SharedMemoryRegion {
            shmem,
            fifo,
            blocks,
        })
    }
}

pub type BlockID = i32;

/// Block size
pub const BLOCK_SIZE: usize = 8192;
/// Max blocks
pub const MAX_BLOCKS: usize = 128;
/// Shared memory size data
pub const SHARED_MEM_SIZE: usize = std::mem::size_of::<FIFOHeader>() + MAX_BLOCKS * std::mem::size_of::<Block>();

/// Block in shared memory.
pub struct Block {
    /// Next block in singly linked list
    pub next: AtomicI64,
    /// Tag in block
    pub tag: mca_btl_base_tag_t,
    /// Message trigger
    pub message_trigger: usize,
    /// Indicates that the block is complete and can be freed
    pub complete: bool,
    /// Amount of data used in the block [0; BLOCK_SIZE]
    pub len: usize,
    /// Actual data in the block
    pub data: [u8; BLOCK_SIZE],
}

impl Block {
    /// Fill the memory location with the given convertor and header data.
    pub unsafe fn fill(
        &mut self,
        convertor: *mut opal_convertor_t,
        header: *mut c_void,
        header_size: usize,
        payload_size: usize,
    ) -> c_int {
        let len = header_size + payload_size;
        let block_data = self.data.as_mut_ptr();
        std::ptr::copy_nonoverlapping(header as *const u8, block_data, header_size);
        if payload_size > 0 {
            let mut data_ptr = std::ptr::null_mut();
            opal_convertor_get_current_pointer_rs(convertor, &mut data_ptr);
            let iov_len = 1;
            let mut iov = iovec {
                iov_base: block_data.offset(header_size.try_into().unwrap()) as *mut _,
                iov_len,
            };
            convert_data(convertor, iov, payload_size);
        }
        self.len = len;
        0
    }

    /// Fill the block with the given data, with reserve space, and returning
    /// the amount of data used in size.
    pub unsafe fn prepare_fill(
        &mut self,
        convertor: *mut opal_convertor_t,
        reserve: usize,
        size: *mut usize,
    ) -> c_int {
        let mut iov = iovec {
            iov_len: *size,
            iov_base: self.data.as_mut_ptr() as *mut _,
        };

        convert_data(convertor, iov, *size);
        0
    }
}

/// Copy or convert the data and store it in the iov buffer.
unsafe fn convert_data(convertor: *mut opal_convertor_t, mut iov: iovec, payload_size: usize) {
    if opal_convertor_need_buffers_rs(convertor) != 0 {
        let mut iov_len = 1;
        let mut iov_len: u32 = iov_len.try_into().unwrap();
        let mut length = 0;
        opal_convertor_pack(convertor, &mut iov, &mut iov_len, &mut length);
        assert_eq!(length, payload_size);
    } else {
        let mut data_ptr = std::ptr::null_mut();
        opal_convertor_get_current_pointer_rs(convertor, &mut data_ptr);
        std::ptr::copy_nonoverlapping(iov.iov_base, data_ptr, payload_size);
    }
}

/// FIFO header in shared memory.
#[repr(C)]
pub struct FIFOHeader {
    pub head: i64,
    pub tail: AtomicI64,
}

/// Indicates a free FIFO entry
pub const FIFO_FREE: i64 = -1;

impl FIFOHeader {
    /// Initialize a FIFO at a memory location.
    pub unsafe fn init(fifo: *mut FIFOHeader) {
        (*fifo).head = FIFO_FREE;
        (*fifo).tail.store(FIFO_FREE, Ordering::Relaxed)
    }
}
