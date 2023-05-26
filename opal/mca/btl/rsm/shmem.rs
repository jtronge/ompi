pub struct SegmentOptions {
    backing_directory: String,
    nodename: String,
    euid: u32,
    jobid: u32,
    node_rank: u32,
}

pub struct Segment;

impl Segment {
    /// Create a new shared memory segment.
    pub fn create(opts: SegmentOptions, size: usize) -> Segment {
        Segment
    }
}

impl Drop for Segment {
    fn drop(&mut self) {
    }
}
