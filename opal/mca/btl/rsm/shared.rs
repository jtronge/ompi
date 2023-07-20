//! Shared memory management code.
use crate::opal::{
    iovec,
    mca_btl_base_descriptor_t,
    mca_btl_base_segment_t,
    mca_btl_base_tag_t,
    opal_convertor_get_current_pointer_rs,
    opal_convertor_need_buffers_rs,
    opal_convertor_pack,
    opal_convertor_t,
};
use crate::{Error, Rank, Result};
use shared_memory::{Shmem, ShmemConf};
use std::os::raw::{c_int, c_void};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicI64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

/// Create a unique path for a shared memory region.
pub fn make_path(node_name: String, rank: Rank, pid: u32) -> PathBuf {
    let rank: u64 = rank.into();
    let pid: u64 = pid.into();
    let time: u64 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    fastrand::seed(rank + pid + time);
    let random: String = (0..16).map(|_| fastrand::alphanumeric()).collect();
    let fname = format!("{}-{}.shmem", node_name, random);
    let mut path = PathBuf::new();
    path.push("/dev/shm");
    path.push(fname);
    path
}

pub struct SharedRegionMap {
    regions: Vec<Option<SharedRegionHandle>>,
}

impl SharedRegionMap {
    pub fn new() -> SharedRegionMap {
        SharedRegionMap { regions: vec![] }
    }

    /// Insert a shared region handle.
    pub fn insert(&mut self, rank: Rank, handle: SharedRegionHandle) {
        let rank: usize = rank.try_into().unwrap();
        if rank >= self.regions.len() {
            self.regions.resize_with(rank + 1, || None);
        }
        let _ = self.regions[rank].insert(handle);
    }

    /// Remove a shared region handle for the given rank.
    pub fn remove(&mut self, rank: Rank) -> Option<SharedRegionHandle> {
        let rank: usize = rank.try_into().unwrap();
        self.regions[rank].take()
    }

    /// Use a mutable region reference in a callback.
    #[inline]
    pub unsafe fn region_mut<F, R>(&self, rank: Rank, f: F) -> R
    where
        F: FnOnce(&mut SharedRegion) -> R,
    {
        // let rank: isize = rank.try_into().unwrap();
        let ptr: *mut Option<SharedRegionHandle> = self.regions.as_ptr() as *mut _;
        let handle_ptr = ptr.offset(rank as isize);
        // let mut handle = self.regions[rank].as_ref().unwrap();
        f((*handle_ptr).as_mut().unwrap().get())
    }

    /// Return a descriptor for a block.
    pub fn init_descriptor(&self, rank: Rank, block_id: BlockID) -> *mut Descriptor {
        unsafe {
            self.region_mut(rank, |region| {
                let block_idx: usize = block_id.try_into().unwrap();
                let block: &mut Block = &mut region.blocks[block_idx];

                // NOTE: We ignore the list super member here
                // Reset the descriptor data
                block.des.base.des_segment_count = 1;
                block.des.base.des_cbfunc = None;
                block.des.base.des_cbdata = std::ptr::null_mut();
                block.des.base.des_context = std::ptr::null_mut();
                block.des.base.des_flags = 0;
                block.des.base.order = 0;
                block.des.rank = rank;
                block.des.block_id = block_id;
                // Set the segment pointers
                block.des.segment.seg_addr.pval = block.data.as_ptr() as *mut _;
                block.des.segment.seg_len = block.len.try_into().unwrap();
                block.des.base.des_segments = &mut block.des.segment as *mut _;
                &mut block.des as *mut _
            })
        }
    }

    /// Reset the descriptor data.
    pub unsafe fn reset_descriptor(&self, des: *mut Descriptor) {
        (*des).base.des_cbfunc = None;
        (*des).base.des_cbdata = std::ptr::null_mut();
        (*des).base.des_context = std::ptr::null_mut();
        (*des).base.des_flags = 0;
        (*des).base.order = 0;
    }
}

/// Data stored in shared memory for a given region.
#[repr(C)]
pub struct SharedRegion {
    pub fifo: FIFOHeader,
    pub blocks: [Block; MAX_BLOCKS],
}

pub const SHARED_REGION_SIZE: usize = std::mem::size_of::<SharedRegion>();

/// Handle around the actual shared memory and pointer to the data.
pub struct SharedRegionHandle {
    _shmem: Shmem,
    region: *mut SharedRegion,
}

impl SharedRegionHandle {
    /// Create a new shared memory path at a given location.
    pub fn create<P: AsRef<Path>>(path: P) -> Result<SharedRegionHandle> {
        unsafe {
            // SAFETY: The initialization below is safe assuming that the
            // shared memory allocates a region of SHARED_REGION_SIZE. Alignment
            // is checked by comparing the pointer with the value returned by
            // align_of().
            let shmem = ShmemConf::new()
                .size(SHARED_REGION_SIZE)
                .flink(path)
                .create();
            let shmem = match shmem {
                Ok(shmem) => shmem,
                Err(err) => return Err(Error::SharedMemoryFailure(err)),
            };

            // Check alignment
            let ptr = shmem.as_ptr();
            assert_eq!((ptr as usize) % std::mem::align_of::<SharedRegion>(), 0);

            // Initialize everything
            std::ptr::write_bytes(ptr, 0, SHARED_REGION_SIZE);
            let region = ptr as *mut SharedRegion;
            (*region).fifo.head.store(FIFO_FREE, Ordering::Release);
            (*region).fifo.tail.store(FIFO_FREE, Ordering::Release);
            // Initialize the complete field just in case
            for block in (*region).blocks.iter_mut() {
                block.complete = false;
            }

            Ok(SharedRegionHandle {
                _shmem: shmem,
                region,
            })
        }
    }

    /// Attach to an existing shared memory path.
    pub fn attach<P: AsRef<Path>>(path: P) -> Result<SharedRegionHandle> {
        let shmem = ShmemConf::new().size(SHARED_REGION_SIZE).flink(path).open();
        let shmem = match shmem {
            Ok(shmem) => shmem,
            Err(err) => return Err(Error::SharedMemoryFailure(err)),
        };

        // Check alignment
        let ptr = shmem.as_ptr();
        assert_eq!((ptr as usize) % std::mem::align_of::<SharedRegion>(), 0);

        let region = ptr as *mut SharedRegion;

        Ok(SharedRegionHandle {
            _shmem: shmem,
            region,
        })
    }

    /// Return a reference to the shared memory region.
    #[inline]
    pub fn get<'a>(&'a mut self) -> &'a mut SharedRegion {
        unsafe {
            // SAFETY: The region pointer is valid since it could only have
            // been created by the create() or the attach() methods above. The
            // returned reference will last only as long as this handle object.
            self.region.as_mut().unwrap()
        }
    }
}

pub type BlockID = i32;

pub const EAGER_LIMIT: usize = 4 * 1024;
/// Block size
pub const BLOCK_SIZE: usize = 32 * 1024;
/// Max blocks
pub const MAX_BLOCKS: usize = 256;

/// Descriptor to return to calling code with block identification info.
#[repr(C)]
pub struct Descriptor {
    pub base: mca_btl_base_descriptor_t,
    pub rank: Rank,
    pub block_id: BlockID,
    /// Internal segment pointer
    segment: mca_btl_base_segment_t,
}

/// Block in shared memory.
#[repr(C)]
pub struct Block {
    /// Internal descriptor data
    des: Descriptor,
    /// Next block in singly linked list
    pub next: AtomicI64,
    /// Tag in block (used for indexing into message callback table)
    pub tag: mca_btl_base_tag_t,
    /// Indicates that the block is complete and can be freed
    pub complete: bool,
    /// Amount of data used in the block [0; BLOCK_SIZE]
    pub len: usize,
    /// Actual data in the block
    pub data: [u8; BLOCK_SIZE],
}

impl Block {
    pub fn descriptor(&mut self) -> *mut Descriptor {
        &mut self.des
    }

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
            let iov_len = 1;
            let iov = iovec {
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
        // TODO: Need to use reserve here?
        assert!((reserve + *size) <= self.data.len());
        let iov = iovec {
            iov_len: *size,
            iov_base: self.data.as_mut_ptr().offset(reserve.try_into().unwrap()) as *mut _,
        };
        convert_data(convertor, iov, *size);
        self.len = reserve + *size;
        0
    }
}

/// Copy or convert the data and store it in the iov buffer.
#[inline]
unsafe fn convert_data(convertor: *mut opal_convertor_t, mut iov: iovec, payload_size: usize) {
    if opal_convertor_need_buffers_rs(convertor) != 0 {
        let iov_len = 1;
        let mut iov_len: u32 = iov_len.try_into().unwrap();
        let mut length = 0;
        opal_convertor_pack(convertor, &mut iov, &mut iov_len, &mut length);
        assert_eq!(length, payload_size);
    } else {
        let mut data_ptr = std::ptr::null_mut();
        opal_convertor_get_current_pointer_rs(convertor, &mut data_ptr);
        std::ptr::copy_nonoverlapping(data_ptr, iov.iov_base, payload_size);
    }
}

/// FIFO header in shared memory.
#[repr(C)]
pub struct FIFOHeader {
    pub head: AtomicI64,
    pub tail: AtomicI64,
}

/// Indicates a free FIFO entry
pub const FIFO_FREE: i64 = -1;
