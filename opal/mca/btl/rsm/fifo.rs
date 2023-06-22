use std::sync::{Arc, Mutex};
use std::sync::atomic::Ordering;
use crate::{Result, Error, Rank};
use crate::shared::{SharedRegionMap, Block, BlockID, FIFOHeader, FIFO_FREE};

pub(crate) struct FIFO {
    map: Arc<Mutex<SharedRegionMap>>,
    pub rank: Rank,
}

impl FIFO {
    pub fn new(map: Arc<Mutex<SharedRegionMap>>, rank: Rank) -> FIFO {
        FIFO {
            map,
            rank,
        }
    }

    /// Pop the block from this FIFO.
    ///
    /// TODO: Should this be marked unsafe since it should only be called by the owning process?
    pub fn pop(&self) -> Option<(Rank, BlockID)> {
        let map = match self.map.lock() {
            Ok(m) => m,
            Err(_) => return None,
        };

        map.region_mut(self.rank, |region| {
            loop {
                if region.fifo.head == FIFO_FREE {
                    return None;
                }

                let old_tail = region.fifo.tail.load(Ordering::SeqCst);
                let old_head = region.fifo.head;
                let (rank, block_id) = extract_rank_block_id(old_head);
                let block_idx: usize = block_id.try_into().unwrap();

                let new_head = if rank == self.rank {
                    let new_head = region.blocks[block_idx].next.load(Ordering::SeqCst);
                    let new_tail = if new_head == FIFO_FREE { FIFO_FREE } else { old_tail };
                    match region.fifo.tail.compare_exchange(
                        old_tail,
                        new_tail,
                        Ordering::SeqCst,
                        Ordering::SeqCst,
                    ) {
                        Ok(_) => new_head,
                        Err(_) => continue,
                    }
                } else {
                    match map.region_mut(rank, |other_region| {
                        let new_head = other_region.blocks[block_idx].next.load(Ordering::SeqCst);
                        let new_tail = if new_head == FIFO_FREE { FIFO_FREE } else { old_tail };
                        region.fifo.tail.compare_exchange(
                            old_tail,
                            new_tail,
                            Ordering::SeqCst,
                            Ordering::SeqCst,
                        ).map(|_| new_head)
                    }) {
                        Ok(head) => head,
                        Err(_) => continue,
                    }
                };
                region.fifo.head = new_head;

                return Some((rank, block_id));
            }
/*
            if region.fifo.head == FIFO_FREE {
                return None;
            }
            let value = region.fifo.head;
            let (rank, block_id) = extract_rank_block_id(value);
            let block_idx: usize = block_id.try_into().unwrap();
            if rank == self.rank {
                // Special case
                pop(value, &mut region.fifo, &mut region.blocks[block_idx]);
            } else {
                map.region_mut(rank, |other_region| {
                    pop(value, &mut region.fifo, &mut other_region.blocks[block_idx]);
                });
            }
            Some((rank, block_id))
*/
        })
    }

    /// Push the block onto this FIFO.
    pub fn push(&self, rank: Rank, block_id: BlockID) -> Result<()> {
        let map = match self.map.lock() {
            Ok(m) => m,
            Err(_) => return Err(Error::LockError),
        };

        // This seems like too much code for what it's trying to do
        map.region_mut(self.rank, |region| {
            loop {
                let new_tail = encode_rank_block_id(rank, block_id);
                let old_tail = region.fifo.tail.load(Ordering::SeqCst);

                if region.fifo.tail.compare_exchange(
                    old_tail,
                    new_tail,
                    Ordering::SeqCst,
                    Ordering::SeqCst,
                ).is_err() {
                    continue;
                }

                if old_tail == FIFO_FREE {
                    region.fifo.head = new_tail;
                } else {
                    let (old_rank, old_block_id) = extract_rank_block_id(old_tail);
                    let old_block_idx: usize = old_block_id.try_into().unwrap();
                    if old_rank == self.rank {
                        region.blocks[old_block_idx].next.store(new_tail, Ordering::SeqCst);
                    } else {
                        map.region_mut(self.rank, |other_region| {
                            other_region.blocks[old_block_idx].next.store(new_tail, Ordering::SeqCst);
                        });
                    }
                }
            }
            Ok(())
/*
            let value = encode_rank_block_id(rank, block_id);

            let block_idx: usize = block_id.try_into().unwrap();
            // See sm_fifo_write_ep() and sm_fifo_write() for the original functions
            if rank == self.rank {
                // Need to grab the block from this region (likely a complete
                // block being returned for freeing)
                let block = &mut region.blocks[block_idx];
                block.next.store(FIFO_FREE, Ordering::SeqCst);
                let prev = region.fifo.tail.swap(value, Ordering::SeqCst);

                assert!(prev != value);

                if prev != FIFO_FREE {
                    let (prev_rank, prev_block_id) = extract_rank_block_id(value);
                    let prev_block_idx: usize = prev_block_id.try_into().unwrap();
                    if prev_rank == self.rank {
                        region.blocks[prev_block_idx].next.store(value, Ordering::SeqCst);
                    } else {
                        map.region_mut(prev_rank, |prev_region| {
                            prev_region.blocks[prev_block_idx].next.store(value, Ordering::SeqCst);
                        });
                    }
                } else {
                    region.fifo.head = value;
                }
            } else {
                // Need to grab the block from a different region
                map.region_mut(rank, |other_region| {
                    let block = &mut other_region.blocks[block_idx];
                    block.next.store(FIFO_FREE, Ordering::SeqCst);
                    let prev = region.fifo.tail.swap(value, Ordering::SeqCst);

                    assert!(prev != value);

                    if prev != FIFO_FREE {
                        let (prev_rank, prev_block_id) = extract_rank_block_id(value);
                        let prev_block_idx: usize = prev_block_id.try_into().unwrap();
                        if prev_rank == self.rank {
                            region.blocks[prev_block_idx].next.store(value, Ordering::SeqCst);
                        } else if prev_rank == rank {
                            other_region.blocks[prev_block_idx].next.store(value, Ordering::SeqCst);
                        } else {
                            map.region_mut(prev_rank, |prev_region| {
                                prev_region.blocks[prev_block_idx].next.store(value, Ordering::SeqCst);
                            });
                        }
                    } else {
                        region.fifo.head = value;
                    }
                });
            }
            Ok(())
*/
        })
    }
}

/// Update the head for a pop operation (see sm_fifo_read() for the original).
fn pop(value: i64, fifo: &mut FIFOHeader, block: &mut Block) {
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

/// Encode the rank and block ID into an i64.
fn encode_rank_block_id(rank: Rank, block_id: BlockID) -> i64 {
    let rank: i64 = rank.try_into().unwrap();
    let block_id: i64 = block_id.try_into().unwrap();
    (rank << 32) | block_id
}
