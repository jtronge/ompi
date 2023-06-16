use std::os::raw::{c_int, c_void};
use std::sync::atomic::AtomicU64;
use crate::opal::{
    iovec,
    opal_convertor_get_current_pointer_rs,
    opal_convertor_need_buffers_rs,
    opal_convertor_pack,
    mca_btl_base_tag_t,
    opal_convertor_t,
};

pub const BLOCK_SIZE: usize = 8192;
pub const MAX_BLOCKS: usize = 128;
pub const SHARED_MEM_SIZE: usize = std::mem::size_of::<FIFO>() + MAX_BLOCKS * std::mem::size_of::<Block>();

pub(crate) struct Block {
    pub(crate) next: isize,
    pub(crate) tag: mca_btl_base_tag_t,
    pub(crate) message_trigger: usize,
    pub(crate) complete: bool,
    pub(crate) len: usize,
    pub(crate) data: [u8; BLOCK_SIZE],
}

impl Block {
    pub(crate) unsafe fn fill(
        &mut self,
        convertor: *mut opal_convertor_t,
        header: *mut c_void,
        header_size: usize,
        payload_size: usize,
    ) -> c_int {
        let len = header_size + payload_size;
        let block_data = self.data.as_mut_ptr();
        std::ptr::copy_nonoverlapping(header as *const u8, block_data, header_size);
        if payload_size > 0 {
            let mut data_ptr = std::ptr::null_mut();
            opal_convertor_get_current_pointer_rs(convertor, &mut data_ptr);
            let iov_len = 1;
            let mut iov = iovec {
                iov_base: block_data.offset(header_size.try_into().unwrap()) as *mut _,
                iov_len,
            };
            convert_data(convertor, iov, payload_size);
        }
        self.len = len;
        0
    }

    /// Fill the block with the given data, with reserve space, and returning
    /// the amount of data used in size.
    pub(crate) unsafe fn prepare_fill(
        &mut self,
        convertor: *mut opal_convertor_t,
        reserve: usize,
        size: *mut usize,
    ) -> c_int {
        let mut iov = iovec {
            iov_len: *size,
            iov_base: self.data.as_mut_ptr() as *mut _,
        };

        convert_data(convertor, iov, *size);
        0
    }
}

/// Copy or convert the data and store it in the iov buffer.
unsafe fn convert_data(convertor: *mut opal_convertor_t, mut iov: iovec, payload_size: usize) {
    if opal_convertor_need_buffers_rs(convertor) != 0 {
        let mut iov_len = 1;
        let mut iov_len: u32 = iov_len.try_into().unwrap();
        let mut length = 0;
        opal_convertor_pack(convertor, &mut iov, &mut iov_len, &mut length);
        assert_eq!(length, payload_size);
    } else {
        let mut data_ptr = std::ptr::null_mut();
        opal_convertor_get_current_pointer_rs(convertor, &mut data_ptr);
        std::ptr::copy_nonoverlapping(iov.iov_base, data_ptr, payload_size);
    }
}

#[repr(C)]
pub(crate) struct FIFO {
    head: u64,
    tail: AtomicU64,
}

impl FIFO {
    /// Initialize a FIFO at a memory location.
    pub(crate) unsafe fn init(fifo: *mut FIFO) {
        // TODO
    }
}
