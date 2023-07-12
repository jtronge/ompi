use crate::fifo::FIFO;
use crate::opal::{obj_construct_rs, opal_list_item_t};
use crate::shared::SharedRegionMap;
use crate::{Rank, Result};
use std::cell::RefCell;
use std::mem::MaybeUninit;
use std::rc::Rc;

/// Info about a given endpoint
#[repr(C)]
pub(crate) struct Endpoint {
    pub _base: opal_list_item_t,
    pub rank: Rank,
    pub fifo: FIFO,
}

impl Endpoint {
    /// Create a new endpoint.
    pub fn new(map: Rc<RefCell<SharedRegionMap>>, rank: Rank) -> Result<Endpoint> {
        let fifo = FIFO::new(Rc::clone(&map), rank);
        let _base = unsafe {
            let mut base = MaybeUninit::uninit();
            obj_construct_rs(base.as_mut_ptr());
            // SAFETY: This was initialized by obj_construct_rs()
            base.assume_init()
        };
        Ok(Endpoint { _base, rank, fifo })
    }
}
