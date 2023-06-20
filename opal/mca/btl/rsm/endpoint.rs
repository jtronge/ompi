use std::mem::MaybeUninit;
use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicI64;
use crate::{Result, Rank};
use crate::shared::{SharedMemoryStore, Block, BlockID, make_path, BLOCK_SIZE, FIFO_FREE};
use crate::fifo::FIFO;
use crate::proc_info;

/// Info about a given endpoint
pub(crate) struct Endpoint {
    pub store: Arc<Mutex<SharedMemoryStore>>,
    pub rank: Rank,
    pub fifo: FIFO,
}

impl Endpoint {
    /// Create a new endpoint.
    pub fn new(store: Arc<Mutex<SharedMemoryStore>>, rank: Rank) -> Result<Endpoint> {
        let path = make_path(&proc_info::node_name(), rank);
        // let shmem = Arc::new(SharedMemoryRegion::attach(path)?);
        let fifo = FIFO::new(Arc::clone(&store), rank);
        Ok(Endpoint {
            store,
            rank,
            fifo,
        })
    }

    /// Use a block located in the shared memory of this endpoint.
    pub(crate) fn use_block<F, R>(&mut self, block_id: BlockID, f: F) -> R
    where
        F: FnOnce(&mut Block) -> R,
    {
        let data = unsafe { MaybeUninit::<[u8; BLOCK_SIZE]>::uninit().assume_init() };
        let mut block = Block {
            next: AtomicI64::new(FIFO_FREE),
            tag: 0,
            message_trigger: 0,
            complete: false,
            data,
            len: 0,
        };
        f(&mut block)
    }
}
