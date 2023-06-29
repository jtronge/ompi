use std::sync::{Arc, Mutex};
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
    // pub map: Arc<Mutex<SharedRegionMap>>,
    pub _base: opal_list_item_t,
    pub rank: Rank,
    pub fifo: FIFO,
}

impl Endpoint {
    /// Create a new endpoint.
    pub fn new(map: Arc<Mutex<SharedRegionMap>>, rank: Rank) -> Result<Endpoint> {
        let fifo = FIFO::new(Arc::clone(&map), rank);
        let _base = unsafe {
            // SAFETY: This will be initialized by obj_construct_rs()
            MaybeUninit::uninit().assume_init()
        };
        let mut ep = Endpoint {
            _base,
            // map,
            rank,
            fifo,
        };
        unsafe {
            obj_construct_rs(&mut ep._base);
        }
        Ok(ep)
    }
}
