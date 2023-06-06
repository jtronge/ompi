use std::ffi::CStr;
use crate::opal::{
    opal_process_info_t,
};

extern {
    static mut opal_process_info: opal_process_info_t;
}

/// Get the number of local procs on this same node
pub(crate) fn num_local_peers() -> u32 {
    unsafe {
        opal_process_info.num_local_peers
    }
}

/// Get the local rank on this node within a job
pub(crate) fn local_rank() -> u16 {
    unsafe {
        opal_process_info.my_local_rank
    }
}

/// Get the node rank
pub(crate) fn node_rank() -> u16 {
    unsafe {
        opal_process_info.my_node_rank
    }
}

/// Get the name of the node
pub(crate) fn node_name() -> String {
    unsafe {
        let cs = CStr::from_ptr(opal_process_info.nodename);
        cs.to_str().unwrap().into()
    }
}
