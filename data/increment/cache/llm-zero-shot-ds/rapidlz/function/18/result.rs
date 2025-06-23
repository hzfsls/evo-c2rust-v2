use std::ptr;

const RAPIDLZ_ERROR_PARAM_UNSUPPORTED: i32 = -1; // Assuming this is the error code

type RapidlzDecompressFunc = fn(*const u8, *mut u8, i32, i32, *const u8, i32) -> i32;

extern "C" {
    fn RapidlzZeroBytesDecode(src: *const u8, compressed_size: i32) -> i32;
    fn RapidlzDecWithPrefixDict(src: *const u8, dst: *mut u8, compressed_size: i32, dst_size: i32, dict_start: *const u8, dict_size: i32) -> i32;
    fn RapidlzDecWithExternalDict(src: *const u8, dst: *mut u8, compressed_size: i32, dst_size: i32, dict_start: *const u8, dict_size: i32) -> i32;
}

pub fn rapidlz_decompress_safe_using_dict(
    src: *const u8,
    dst: *mut u8,
    compressed_size: i32,
    dst_size: i32,
    dict_start: *const u8,
    dict_size: i32,
) -> i32 {
    if src.is_null() || compressed_size == 0 || dst.is_null() || dst_size < 0 {
        return RAPIDLZ_ERROR_PARAM_UNSUPPORTED;
    }

    if dst_size == 0 {
        unsafe {
            return RapidlzZeroBytesDecode(src, compressed_size);
        }
    }

    let rapidlz_dec_func: RapidlzDecompressFunc = if dict_size == 0 || unsafe { dict_start.add(dict_size as usize) } == dst {
        RapidlzDecWithPrefixDict
    } else {
        RapidlzDecWithExternalDict
    };

    unsafe {
        rapidlz_dec_func(src, dst, compressed_size, dst_size, dict_start, dict_size)
    }
}
