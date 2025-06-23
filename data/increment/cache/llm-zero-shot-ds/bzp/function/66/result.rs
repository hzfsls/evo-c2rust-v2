use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use std::ptr;

#[repr(C)]
pub enum BzpError {
    Ok = 0,
    ErrorParam = -1,
    ErrorIo = -2,
}

#[repr(C)]
pub struct BzpFile {
    file_ptr: *mut File,
}

#[repr(C)]
pub struct BzpCompressFile {
    input: *mut BzpFile,
    output: *mut BzpFile,
}

#[repr(C)]
pub struct BzpAlgorithmInfo {
    compress_file: *mut BzpCompressFile,
}

extern "C" fn BzpOpenFile(bzp_info: *mut BzpAlgorithmInfo, in_name: *const c_char, out_name: *const c_char) -> i32 {
    if bzp_info.is_null() {
        return BzpError::ErrorParam as i32;
    }

    unsafe {
        let in_name_cstr = CStr::from_ptr(in_name);
        let out_name_cstr = CStr::from_ptr(out_name);
        
        let in_name_str = match in_name_cstr.to_str() {
            Ok(s) => s,
            Err(_) => return BzpError::ErrorParam as i32,
        };
        
        let out_name_str = match out_name_cstr.to_str() {
            Ok(s) => s,
            Err(_) => return BzpError::ErrorParam as i32,
        };

        let input_file = match File::open(in_name_str) {
            Ok(f) => f,
            Err(_) => {
                return BzpError::ErrorIo as i32;
            }
        };

        let output_file = match File::create(out_name_str) {
            Ok(f) => f,
            Err(_) => {
                return BzpError::ErrorIo as i32;
            }
        };

        (*bzp_info).compress_file.input.file_ptr = Box::into_raw(Box::new(input_file));
        (*bzp_info).compress_file.output.file_ptr = Box::into_raw(Box::new(output_file));

        BzpError::Ok as i32
    }
}
