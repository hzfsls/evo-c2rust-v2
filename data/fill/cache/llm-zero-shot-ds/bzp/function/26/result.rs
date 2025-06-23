use std::alloc::{alloc, dealloc, Layout};
use std::ptr::null_mut;

#[repr(C)]
pub struct BzpOutComdata {
    pub out: *mut u8,
    pub n_buf: i32,
    pub buf: i32,
    pub num: i32,
    pub block_size: i32,
}

const BZP_BASE_BLOCK_SIZE: usize = 1; // Assuming a default value if not provided

pub fn bzp_out_com_data_init(block_size: i32) -> *mut BzpOutComdata {
    // Allocate memory for BzpOutComdata
    let layout = Layout::new::<BzpOutComdata>();
    let out_data = unsafe { alloc(layout) as *mut BzpOutComdata };
    
    if out_data.is_null() {
        return null_mut();
    }

    // Initialize the struct fields
    unsafe {
        (*out_data).out = null_mut();
        (*out_data).n_buf = 0;
        (*out_data).buf = 0;
        (*out_data).num = 0;
        (*out_data).block_size = block_size;
    }

    // Allocate memory for the out buffer
    let buffer_size = (block_size as usize) * BZP_BASE_BLOCK_SIZE * std::mem::size_of::<u32>();
    let buffer_layout = Layout::from_size_align(buffer_size, std::mem::align_of::<u8>())
        .expect("Failed to create layout for buffer");
    let buffer = unsafe { alloc(buffer_layout) as *mut u8 };

    if buffer.is_null() {
        // Clean up previously allocated memory if buffer allocation fails
        unsafe { dealloc(out_data as *mut u8, layout) };
        return null_mut();
    }

    unsafe {
        (*out_data).out = buffer;
    }

    out_data
}
