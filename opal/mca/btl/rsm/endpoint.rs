use crate::shared::{BLOCK_SIZE, Block};
use std::mem::MaybeUninit;

/// Info about a given endpoint
pub(crate) struct Endpoint {
    pub(crate) local_rank: u16,
}

impl Endpoint {
    pub(crate) fn push(&mut self, block_id: isize) -> Result<(), ()> {
        Ok(())
    }

    pub(crate) fn use_block<F, R>(&mut self, block_id: isize, f: F) -> R
    where
        F: FnOnce(&mut Block) -> R,
    {
        let data = unsafe { MaybeUninit::<[u8; BLOCK_SIZE]>::uninit().assume_init() };
        let mut block = Block {
            next: 0,
            tag: 0,
            message_trigger: 0,
            complete: false,
            data,
            len: 0,
        };
        f(&mut block)
    }
}
