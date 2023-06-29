//! Shared memory management code.
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicI64, Ordering};
use std::mem::MaybeUninit;
use std::os::raw::{c_void, c_int};
use std::cell::RefCell;
use shared_memory::{ShmemConf, Shmem};
use log::debug;
use crate::{Result, Error, Rank};
use crate::opal::{
    mca_btl_base_descriptor_t,
    mca_btl_base_segment_t,
    mca_btl_base_tag_t,
    iovec,
    opal_ptr_t,
    opal_convertor_t,
    opal_convertor_get_current_pointer_rs,
    opal_convertor_need_buffers_rs,
    opal_convertor_pack,
    opal_free_list_item_t,
};

/// Create a unique path for a shared memory region.
pub fn make_path(node_name: String, rank: Rank, pid: u32) -> PathBuf {
    let rank: u64 = rank.into();
    let pid: u64 = pid.into();
    fastrand::seed(rank + pid);
    let random: String = (0..16).map(|_| fastrand::alphanumeric()).collect();
    let fname = format!("{}-{}.shmem", node_name, random);
    let mut path = PathBuf::new();
    path.push("/dev/shm");
    path.push(fname);
    path
}

pub struct SharedRegionMap {
    pub regions: HashMap<Rank, RefCell<SharedRegionHandle>>,
}

impl SharedRegionMap {
    /// Use a mutable region reference in a callback.
    pub fn region_mut<F, R>(&self, rank: Rank, f: F) -> R
    where
        F: FnOnce(&mut SharedRegion) -> R,
    {
        let mut handle = self.regions.get(&rank).unwrap().borrow_mut();
        f(handle.get())
    }

    /// Return a descriptor for a block.
    pub fn descriptor(&self, rank: Rank, block_id: BlockID) -> Descriptor {
        let mut handle = self.regions.get(&rank).unwrap().borrow_mut();
        let region = handle.get();
        let block_idx: usize = block_id.try_into().unwrap();
        let block: &mut Block = &mut region.blocks[block_idx];
        let segment = Box::new(mca_btl_base_segment_t {
            seg_addr: opal_ptr_t {
                pval: block.data.as_ptr() as *mut _,
            },
            seg_len: block.len.try_into().unwrap(),
        });
        let des_segments = Box::into_raw(segment);

        // SAFETY: This parameter does not seem to be getting initialized by
        // any of the other BTLs so here we just leave it uninitilized, but this
        // is UB.
        let super_ = unsafe {
            MaybeUninit::<opal_free_list_item_t>::uninit().assume_init()
        };
        Descriptor {
            base: mca_btl_base_descriptor_t {
                super_,
                des_segments,
                des_segment_count: 1,
                des_cbfunc: None,
                des_cbdata: std::ptr::null_mut(),
                des_context: std::ptr::null_mut(),
                des_flags: 0,
                order: 0,
            },
            rank,
            block_id,
        }
    }
}

/// Descriptor to return to calling code with block identification info.
#[repr(C)]
pub struct Descriptor {
    pub base: mca_btl_base_descriptor_t,
    pub rank: Rank,
    pub block_id: BlockID,
}

impl Drop for Descriptor {
    fn drop(&mut self) {
        unsafe {
            let _ = Box::from_raw(self.base.des_segments);
        }
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
            (*region).fifo.head = FIFO_FREE;
            (*region).fifo.tail.store(FIFO_FREE, Ordering::Relaxed);
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
        let shmem = ShmemConf::new()
            .size(SHARED_REGION_SIZE)
            .flink(path)
            .open();
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

/// Block size
pub const BLOCK_SIZE: usize = 8192;
/// Max blocks
pub const MAX_BLOCKS: usize = 128;

/// Block in shared memory.
#[derive(Debug)]
#[repr(C)]
pub struct Block {
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
        assert!((reserve + *size) < self.data.len());
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
        debug!("copying data pointer here: {:x}", data_ptr as usize);
        std::ptr::copy_nonoverlapping(data_ptr, iov.iov_base, payload_size);
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
