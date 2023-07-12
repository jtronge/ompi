use crate::opal::{
    opal_modex_recv_string_rs, opal_modex_recv_value_rs, opal_modex_send_string_rs,
    opal_process_name_t, OPAL_SUCCESS, PMIX_LOCAL, PMIX_LOCAL_RANK, PMIX_UINT16,
};
use crate::{Error, Result};
use log::debug;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

/// Receive local rank modex data from another process. Uses the
/// OPAL_MODEX_RECV_VALUE macro in C.
pub fn recv_local_rank(proc_name: &opal_process_name_t) -> Result<u16> {
    let mut rank = 0;

    // This should be safe since all of the pointers here are valid.
    let rc = unsafe {
        opal_modex_recv_value_rs(
            PMIX_LOCAL_RANK.as_ptr() as *const _,
            proc_name,
            (&mut rank as *mut u16) as *mut _,
            PMIX_UINT16,
        )
    };

    if rc == OPAL_SUCCESS {
        Ok(rank)
    } else {
        Err(Error::OpalError(rc))
    }
}

/// Receive a string value from some process.
pub fn recv_string(key: &str, proc_name: &opal_process_name_t) -> Result<String> {
    unsafe {
        let key = CString::new(key.to_string()).unwrap();
        let key_nul = key.as_bytes_with_nul();
        let mut ptr: *mut c_char = std::ptr::null_mut();

        let rc = opal_modex_recv_string_rs(
            key_nul.as_ptr() as *const _,
            proc_name,
            (&mut ptr as *mut *mut c_char) as *mut _,
        );

        if rc == OPAL_SUCCESS {
            debug!("ptr: {:?}", ptr);
            if !ptr.is_null() {
                let value = CStr::from_ptr(ptr)
                    .to_str()
                    .expect("Failed to convert modex C string to Rust string")
                    .to_string();
                Ok(value)
            } else {
                Err(Error::ModexValueRecvFailure)
            }
        } else {
            Err(Error::OpalError(rc))
        }
    }
}

/// Publish a string value for the PMIX_LOCAL scope.
pub fn send_string_local(key: &str, value: &str) -> Result<()> {
    let key = CString::new(key.to_string()).unwrap();
    let key_nul = key.as_bytes_with_nul();
    let value = CString::new(value.to_string()).unwrap();
    let value_nul = value.as_bytes_with_nul();

    let rc = unsafe {
        opal_modex_send_string_rs(
            PMIX_LOCAL,
            key_nul.as_ptr() as *const _,
            value_nul.as_ptr() as *mut _,
            value_nul.len(),
        )
    };

    if rc == OPAL_SUCCESS {
        Ok(())
    } else {
        Err(Error::OpalError(rc))
    }
}
