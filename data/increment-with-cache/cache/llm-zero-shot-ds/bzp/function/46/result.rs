use std::alloc::{alloc, Layout};
use libc::c_void;

#[repr(C)]
pub struct InDeComdata {
    input: *mut c_void,
    output: *mut c_void,
    num: i32,
    lasChar: i32,
    nBuf: i32,
    buf: i32,
    blockCRC: u32,
}

pub const BZP_ASCII_SIZE: i32 = /* value */;
pub const BZP_INIT_BLOCK_CRC: u32 = /* value */;

pub fn BzpInDeComdataInit() -> *mut InDeComdata {
    let layout = Layout::new::<InDeComdata>();
    let in_data = unsafe { alloc(layout) as *mut InDeComdata };
    
    if in_data.is_null() {
        return std::ptr::null_mut();
    }

    unsafe {
        (*in_data).input = std::ptr::null_mut();
        (*in_data).output = std::ptr::null_mut();
        (*in_data).num = 0;
        (*in_data).lasChar = BZP_ASCII_SIZE;
        (*in_data).nBuf = 0;
        (*in_data).buf = 0;
        (*in_data).blockCRC = BZP_INIT_BLOCK_CRC;
    }

    in_data
}
