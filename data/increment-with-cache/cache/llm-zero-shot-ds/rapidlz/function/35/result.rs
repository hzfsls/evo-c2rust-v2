use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_ushort, c_void};
use std::path::Path;
use std::ptr;
use libc::{strdup, free, basename};
use std::mem;

type RapidlzLogFunc = extern "C" fn(*const c_char, usize);

static mut g_rapidlzLogFunc: Option<RapidlzLogFunc> = None;

const LOG_BUF_SIZE: usize = 1024; // Assuming a typical buffer size, adjust as needed

pub unsafe fn rapidlz_log_write(error_code: usize, file_name: *const c_char, line: c_ushort, fmt: *const c_char, ...) {
    let func = g_rapidlzLogFunc;
    if func.is_none() {
        return;
    }
    let func = func.unwrap();

    let filename = strdup(file_name);
    if filename.is_null() {
        return;
    }

    let mut output = [0u8; LOG_BUF_SIZE];
    let base_name = basename(filename);
    let ret_val = libc::snprintf(
        output.as_mut_ptr() as *mut c_char,
        LOG_BUF_SIZE,
        b"\n[Rapidlz-Log] File=%s, Line=%u, Error=%zu\n\0".as_ptr() as *const c_char,
        base_name,
        line,
        error_code
    );

    if ret_val < 0 {
        free(filename);
        return;
    }
    let len = ret_val as usize;

    free(filename);

    let mut alist: libc::va_list;
    unsafe {
        alist = mem::zeroed();
        libc::va_start(alist, fmt);
    }

    let ret_val = libc::vsnprintf(
        (output.as_mut_ptr() as *mut c_char).add(len),
        LOG_BUF_SIZE - len,
        fmt,
        alist
    );

    unsafe {
        libc::va_end(alist);
    }

    if ret_val < 0 {
        return;
    }

    let output_str = CStr::from_ptr(output.as_ptr() as *const c_char);
    func(output.as_ptr() as *const c_char, output_str.to_bytes().len() + 1);
}
