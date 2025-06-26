use std::alloc::{alloc, Layout};
use std::ptr::null_mut;

#[repr(C)]
pub struct BzpStream {
    file_ptr: *mut std::ffi::c_void,
    pos: usize,
    n_buf: usize,
}

pub fn bzp_stream_init() -> *mut BzpStream {
    let layout = Layout::new::<BzpStream>();
    unsafe {
        let stream = alloc(layout) as *mut BzpStream;
        if stream.is_null() {
            return null_mut();
        }
        (*stream).file_ptr = null_mut();
        (*stream).pos = 0;
        (*stream).n_buf = 0;
        stream
    }
}
