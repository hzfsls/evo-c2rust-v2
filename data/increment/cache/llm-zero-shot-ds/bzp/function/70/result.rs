use std::alloc::{alloc, Layout};
use std::ptr::null_mut;

#[repr(C)]
pub struct BzpOutComdata {
    out: *mut u8,
    n_buf: u32,
    buf: u32,
    num: u32,
    block_size: i32,
}

pub const BZP_BASE_BLOCK_SIZE: usize = 4; // Assuming BZP_BASE_BLOCK_SIZE is 4 (size of u32)

pub fn bzp_out_com_data_init(block_size: i32) -> *mut BzpOutComdata {
    // Allocate memory for BzpOutComdata
    let layout = Layout::new::<BzpOutComdata>();
    let out_data = unsafe { alloc(layout) as *mut BzpOutComdata };
    
    if out_data.is_null() {
        return null_mut();
    }

    // Initialize fields
    unsafe {
        (*out_data).out = null_mut();
        (*out_data).n_buf = 0;
        (*out_data).buf = 0;
        (*out_data).num = 0;
        (*out_data).block_size = block_size;
    }

    // Allocate memory for out buffer
    let out_buffer_size = block_size as usize * BZP_BASE_BLOCK_SIZE;
    let out_layout = Layout::array::<u8>(out_buffer_size).unwrap();
    let out_buffer = unsafe { alloc(out_layout) as *mut u8 };

    if out_buffer.is_null() {
        unsafe {
            std::alloc::dealloc(out_data as *mut u8, layout);
        }
        return null_mut();
    }

    unsafe {
        (*out_data).out = out_buffer;
    }

    out_data
}
