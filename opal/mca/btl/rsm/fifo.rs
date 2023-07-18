use crate::shared::{BlockID, Block, SharedRegionMap, FIFO_FREE, FIFO_LOCK};
use crate::{Error, Rank, Result};
use log::info;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::Ordering;

pub(crate) struct FIFO {
    map: Rc<RefCell<SharedRegionMap>>,
    pub rank: Rank,
}

impl FIFO {
    pub fn new(map: Rc<RefCell<SharedRegionMap>>, rank: Rank) -> FIFO {
        FIFO { map, rank }
    }

    /// Pop the block from this FIFO.
    #[inline]
    pub unsafe fn pop(&self) -> Option<(Rank, BlockID)> {
        let map = match self.map.try_borrow_mut() {
            Ok(m) => m,
            Err(_) => return None,
        };

        map.region_mut(self.rank, |region| {
            if region.fifo.head.load(Ordering::Acquire) == FIFO_FREE {
                return None;
            }

            let value = region.fifo.head.load(Ordering::Acquire);
            let (rank, block_id) = extract_rank_block_id(value);
            let block: *mut Block = if rank == self.rank {
                &mut region.blocks[block_id as usize] as *mut _
            } else {
                map.region_mut(rank, |oregion| &mut oregion.blocks[block_id as usize] as *mut _)
            };

            region.fifo.head.store(FIFO_FREE, Ordering::Relaxed);

            assert_ne!((*block).next.load(Ordering::Relaxed), value);

            if (*block).next.load(Ordering::Acquire) == FIFO_FREE {
                if region.fifo.tail.compare_exchange(value, FIFO_FREE, Ordering::Release, Ordering::Acquire).is_err() {
                    while (*block).next.load(Ordering::Acquire) == FIFO_FREE { }
                    region.fifo.head.store((*block).next.load(Ordering::Acquire), Ordering::Release);
                }
            } else {
                region.fifo.head.store((*block).next.load(Ordering::Acquire), Ordering::Release);
            }

            Some((rank, block_id))
/*
            loop {
                let old_head = region.fifo.head;
                let old_tail = region.fifo.tail.load(Ordering::SeqCst);
                if old_head == FIFO_FREE {
                    return None;
                }

                if old_tail == FIFO_LOCK {
                    continue;
                }

                // We lock the tail always
                if region
                    .fifo
                    .tail
                    .compare_exchange(old_tail, FIFO_LOCK, Ordering::SeqCst, Ordering::SeqCst)
                    .is_err()
                {
                    continue;
                }

                let (rank, block_id) = extract_rank_block_id(old_head);
                let block_idx: usize = block_id.try_into().unwrap();

                let new_head = if rank == self.rank {
                    region.blocks[block_idx].next
                } else {
                    map.region_mut(rank, |other_region| other_region.blocks[block_idx].next)
                };
                region.fifo.head = new_head;
                let new_tail = if new_head == FIFO_FREE {
                    FIFO_FREE
                } else {
                    old_tail
                };

                // Unlock
                region.fifo.tail.store(new_tail, Ordering::SeqCst);

                return Some((rank, block_id));
            }
*/
        })
    }

    /// Push the block onto this FIFO.
    #[inline]
    pub unsafe fn push(&self, rank: Rank, block_id: BlockID) -> Result<()> {
        let map = match self.map.try_borrow_mut() {
            Ok(m) => m,
            Err(_) => return Err(Error::LockError),
        };

        // This seems like too much code for what it's trying to do
        let value = encode_rank_block_id(rank, block_id);
        map.region_mut(self.rank, |region| {
            let prev = region.fifo.tail.swap(value, Ordering::AcqRel);

            if prev != FIFO_FREE {
                info!("prev: {:?}, value: {:?}", extract_rank_block_id(prev), extract_rank_block_id(value));
            } else {
                info!("value: {:?}", extract_rank_block_id(value));
            }
            assert_ne!(prev, value);

            if prev != FIFO_FREE {
                let (prev_rank, prev_block_id) = extract_rank_block_id(prev);
                let block: *mut Block = if prev_rank == self.rank {
                    &mut region.blocks[prev_block_id as usize] as *mut _
                } else {
                    map.region_mut(prev_rank, |oregion| &mut oregion.blocks[prev_block_id as usize] as *mut _)
                };
                (*block).next.store(value, Ordering::Release);
            } else {
                region.fifo.head.store(value, Ordering::Release);
            }

            Ok(())
/*
            loop {
                let new_tail = encode_rank_block_id(rank, block_id);
                let old_tail = region.fifo.tail.load(Ordering::SeqCst);

                if old_tail == FIFO_LOCK {
                    continue;
                }

                // Lock
                if region
                    .fifo
                    .tail
                    .compare_exchange(old_tail, FIFO_LOCK, Ordering::SeqCst, Ordering::SeqCst)
                    .is_err()
                {
                    continue;
                }

                if old_tail == FIFO_FREE {
                    region.fifo.head = new_tail;
                } else {
                    let (old_rank, old_block_id) = extract_rank_block_id(old_tail);
                    let old_block_idx: usize = old_block_id.try_into().unwrap();
                    if old_rank == self.rank {
                        region.blocks[old_block_idx].next = new_tail;
                    } else {
                        map.region_mut(old_rank, |other_region| {
                            other_region.blocks[old_block_idx].next = new_tail;
                        });
                    }
                }

                // Ensure next is FIFO_FREE
                let block_idx: usize = block_id.try_into().unwrap();
                if rank == self.rank {
                    region.blocks[block_idx].next = FIFO_FREE;
                } else {
                    map.region_mut(rank, |other_region| {
                        other_region.blocks[block_idx].next = FIFO_FREE;
                    });
                }

                // Unlock
                region.fifo.tail.store(new_tail, Ordering::SeqCst);

                return Ok(());
            }
*/
        })
    }
}

/// Extract the rank and block ID from an i64.
#[inline]
fn extract_rank_block_id(value: i64) -> (Rank, BlockID) {
    let rank = (value >> 32).try_into().unwrap();
    let block_id = (value & 0xFFFFFFFF).try_into().unwrap();
    (rank, block_id)
}

/// Encode the rank and block ID into an i64.
#[inline]
fn encode_rank_block_id(rank: Rank, block_id: BlockID) -> i64 {
    let rank: i64 = rank.try_into().unwrap();
    let block_id: i64 = block_id.try_into().unwrap();
    (rank << 32) | block_id
}
