use std::os::raw::{c_char, c_int, c_void};
use crate::opal::{
    opal_modex_recv_value_rs,
    opal_process_name_t,
    PMIX_INT8,
    PMIX_INT16,
    PMIX_INT32,
    PMIX_INT64,
    PMIX_UINT8,
    PMIX_UINT16,
    PMIX_UINT32,
    PMIX_UINT64,
    PMIX_FLOAT,
    PMIX_DOUBLE,
    PMIX_LOCAL_RANK,
};

pub enum Key {
    LocalRank,
}

impl Key {
    fn cstr_key(&self) -> *const c_char {
        match self {
            Key::LocalRank => PMIX_LOCAL_RANK.as_ptr() as *const _,
        }
    }
}

pub trait Modex {
    fn data_type() -> u32;
    fn ptr_mut(&mut self) -> *mut c_void;
}

macro_rules! impl_modex {
    ($ty:ident, $type_value:ident) => {
        impl Modex for $ty {
            fn data_type() -> u32 {
                $type_value
            }

            fn ptr_mut(&mut self) -> *mut c_void {
                (self as *mut $ty) as *mut _
            }
        }
    }
}

impl_modex!(i8, PMIX_INT8);
impl_modex!(i16, PMIX_INT16);
impl_modex!(i32, PMIX_INT32);
impl_modex!(i64, PMIX_INT64);
impl_modex!(u8, PMIX_UINT8);
impl_modex!(u16, PMIX_UINT16);
impl_modex!(u32, PMIX_UINT32);
impl_modex!(u64, PMIX_UINT64);
impl_modex!(f32, PMIX_FLOAT);
impl_modex!(f64, PMIX_DOUBLE);

/// Receive modex data from another process. Uses the OPAL_MODEX_RECV_VALUE
/// macro in C.
pub fn recv_value<T>(
    key: Key,
    proc_name: &opal_process_name_t,
    data: &mut T,
) -> c_int
where
    T: Modex,
{
    unsafe {
        // This should be safe since all of the pointers here are valid.
        opal_modex_recv_value_rs(
            key.cstr_key(),
            proc_name,
            data.ptr_mut(),
            T::data_type(),
        )
    }
}
