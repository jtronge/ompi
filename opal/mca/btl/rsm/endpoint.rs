use std::sync::Mutex;
use std::rc::Rc;
use std::mem::MaybeUninit;
use crate::{Result, Rank};
use crate::opal::{
    opal_list_item_t,
    obj_construct_rs,
};
use crate::shared::SharedRegionMap;
use crate::fifo::FIFO;

/// Info about a given endpoint
#[repr(C)]
pub(crate) struct Endpoint {
    pub _base: opal_list_item_t,
    pub rank: Rank,
    pub fifo: FIFO,
}

impl Endpoint {
    /// Create a new endpoint.
    pub fn new(map: Rc<Mutex<SharedRegionMap>>, rank: Rank) -> Result<Endpoint> {
        let fifo = FIFO::new(Rc::clone(&map), rank);
        let _base = unsafe {
            let mut base = MaybeUninit::uninit();
            obj_construct_rs(base.as_mut_ptr());
            // SAFETY: This was initialized by obj_construct_rs()
            base.assume_init()
        };
        Ok(Endpoint {
            _base,
            rank,
            fifo,
        })
    }
}
