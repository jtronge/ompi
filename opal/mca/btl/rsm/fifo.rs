use std::sync::{Arc, Mutex};
use std::sync::atomic::Ordering;
use crate::{Result, Rank};
use crate::shared::{SharedMemoryStore, Block, BlockID, FIFOHeader, FIFO_FREE};

pub(crate) struct FIFO {
    store: Arc<Mutex<SharedMemoryStore>>,
    pub rank: Rank,
}

impl FIFO {
    pub fn new(store: Arc<Mutex<SharedMemoryStore>>, rank: Rank) -> FIFO {
        FIFO {
            store,
            rank,
        }
    }

    pub fn pop(&self) -> Option<(Rank, BlockID)> {
        unsafe {
            let store = match self.store.lock() {
                Ok(store) => store,
                Err(_) => return None,
            };
            let mut region = store.regions.get(&self.rank).unwrap().borrow_mut();

            let fifo = region.fifo;
            if (*fifo).head == FIFO_FREE {
                return None;
            }
            let value = (*fifo).head;
            let (rank, block_id) = extract_rank_block_id(value);
            let block_idx: isize = block_id.try_into().unwrap();
            if rank == self.rank {
                let block = region.blocks.offset(block_idx);
                // Special case
                update_head(value, fifo.as_mut().unwrap(), block.as_mut().unwrap());
            } else {
                let mut other_region = store.regions.get(&rank).unwrap().borrow_mut();
                let block = other_region.blocks.offset(block_idx);
                update_head(value, fifo.as_mut().unwrap(), block.as_mut().unwrap());
            }
            Some((rank, block_id))
        }
    }

    pub fn push(&self, rank: Rank, block_id: BlockID) -> Result<()> {
        Ok(())
    }
}

/// Update the head for a pop operation (see sm_fifo_read() for the original).
fn update_head(value: i64, fifo: &mut FIFOHeader, block: &mut Block) {
    if block.next.load(Ordering::SeqCst) == FIFO_FREE {
        if fifo.tail.compare_exchange(value, FIFO_FREE, Ordering::SeqCst, Ordering::SeqCst).is_err() {
            while block.next.load(Ordering::SeqCst) == FIFO_FREE {}
            fifo.head = block.next.load(Ordering::SeqCst);
        }
    } else {
        fifo.head = block.next.load(Ordering::SeqCst);
    }
}

/// Extract the rank and block ID from an i64.
fn extract_rank_block_id(value: i64) -> (Rank, BlockID) {
    let rank = (value >> 32).try_into().unwrap();
    let block_id = (value & 0xFFFFFFFF).try_into().unwrap();
    (rank, block_id)
}
