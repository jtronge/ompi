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
}
