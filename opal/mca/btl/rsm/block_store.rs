use std::sync::{Arc, Mutex};
use std::mem::MaybeUninit;
use crate::Rank;
use crate::shared::{SharedMemoryStore, Block, BlockID, BLOCK_SIZE, MAX_BLOCKS};

pub(crate) struct BlockStore {
    store: Arc<Mutex<SharedMemoryStore>>,
    rank: Rank,
    free_blocks: Vec<BlockID>,
}

impl BlockStore {
    pub fn new(store: Arc<Mutex<SharedMemoryStore>>, rank: Rank) -> BlockStore {
        let free_blocks = (0..MAX_BLOCKS).map(|block_id| block_id.try_into().unwrap()).collect();
        BlockStore {
            store,
            rank,
            free_blocks,
        }
    }

    /// Allocate a new block
    pub fn alloc(&mut self) -> Option<BlockID> {
        self.free_blocks.pop()
    }

    /// Free a block
    pub fn free(&mut self, block_id: BlockID) {
        self.free_blocks.push(block_id);
    }
}
