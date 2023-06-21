use std::sync::{Arc, Mutex};
use crate::{Result, Rank};
use crate::shared::SharedRegionMap;
use crate::fifo::FIFO;

/// Info about a given endpoint
pub(crate) struct Endpoint {
    // pub map: Arc<Mutex<SharedRegionMap>>,
    pub rank: Rank,
    pub fifo: FIFO,
}

impl Endpoint {
    /// Create a new endpoint.
    pub fn new(map: Arc<Mutex<SharedRegionMap>>, rank: Rank) -> Result<Endpoint> {
        let fifo = FIFO::new(Arc::clone(&map), rank);
        Ok(Endpoint {
            // map,
            rank,
            fifo,
        })
    }

/*
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
*/
}
