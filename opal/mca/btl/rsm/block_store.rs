use crate::shared::{BlockID, SharedRegionMap, MAX_BLOCKS};

pub(crate) struct BlockStore {
    _map: *mut SharedRegionMap,
    free_blocks: Vec<BlockID>,
}

impl BlockStore {
    pub fn new(map: *mut SharedRegionMap) -> BlockStore {
        let free_blocks = (0..MAX_BLOCKS)
            .map(|block_id| block_id.try_into().unwrap())
            .collect();
        BlockStore {
            _map: map,
            free_blocks,
        }
    }

    /// Allocate a new block
    #[inline]
    pub fn alloc(&mut self) -> Option<BlockID> {
        self.free_blocks.pop()
    }

    /// Free a block
    #[inline]
    pub fn free(&mut self, block_id: BlockID) {
        self.free_blocks.push(block_id);
    }
}
