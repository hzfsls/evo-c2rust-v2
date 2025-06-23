use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_ushort, c_void};
use std::mem;
use std::ptr;
use std::fmt;
use std::panic;

type CmptlzLogFunc = extern "C" fn(*const c_char, usize);

static mut G_CMPTLZ_LOG_FUNC: Option<CmptlzLogFunc> = None;

const LOG_BUF_SIZE: usize = 1024; // Assuming a default buffer size

pub unsafe extern "C" fn cmptlz_log_write(
    error_code: usize,
    func_name: *const c_char,
    line: c_ushort,
    fmt: *const c_char,
    ...
) {
    let mut alist: va_list::VaList;
    let mut output = [0u8; LOG_BUF_SIZE];
    let func = G_CMPTLZ_LOG_FUNC;

    if func.is_none() {
        return;
    }
    let func = func.unwrap();

    // Convert func_name to Rust string safely
    let func_name_str = if func_name.is_null() {
        "(null)"
    } else {
        match CStr::from_ptr(func_name).to_str() {
            Ok(s) => s,
            Err(_) => "(invalid)",
        }
    };

    // Format the first part of the message
    let ret = snprintf_s(
        output.as_mut_ptr() as *mut c_char,
        LOG_BUF_SIZE,
        LOG_BUF_SIZE - 1,
        b"\n[Cmptlz-Log] Func=%s, Line=%u, Error=0x%zx\n\0".as_ptr() as *const c_char,
        func_name_str,
        line,
        error_code,
    );
    if ret < 0 {
        return;
    }
    let len = ret as usize;

    // Handle variadic arguments
    alist = va_list::VaList::new();
    let fmt_str = if fmt.is_null() {
        "(null)"
    } else {
        match CStr::from_ptr(fmt).to_str() {
            Ok(s) => s,
            Err(_) => "(invalid)",
        }
    };

    let ret = vsnprintf_s(
        output[len..].as_mut_ptr() as *mut c_char,
        LOG_BUF_SIZE - len,
        LOG_BUF_SIZE - len - 1,
        fmt_str,
        alist,
    );
    va_list::VaList::end(&mut alist);
    if ret < 0 {
        return;
    }

    // Call the log function
    let output_str = match CStr::from_ptr(output.as_ptr() as *const c_char).to_str() {
        Ok(s) => s,
        Err(_) => return,
    };
    func(output.as_ptr() as *const c_char, output_str.len() + 1);
}

// Helper functions (would need proper implementations or bindings)
extern "C" {
    fn snprintf_s(
        buffer: *mut c_char,
        size_of_buffer: usize,
        max_count: usize,
        format: *const c_char,
        ...
    ) -> i32;
    
    fn vsnprintf_s(
        buffer: *mut c_char,
        size_of_buffer: usize,
        max_count: usize,
        format: *const c_char,
        args: va_list::VaList,
    ) -> i32;
}

// VaList implementation would need to be provided
mod va_list {
    pub struct VaList(*mut ());

    impl VaList {
        pub unsafe fn new() -> Self {
            // Implementation depends on platform
            unimplemented!()
        }

        pub unsafe fn end(&mut self) {
            // Implementation depends on platform
        }
    }
}
