use std::alloc::{alloc, dealloc, Layout};
use std::ptr::{null_mut, null};
use std::mem::size_of;
use std::os::raw::{c_void, c_int};

// Assuming these constants are defined elsewhere
const RAPIDLZ_INPUT_INVALID: i32 = 0;
const RAPIDLZ_MALLOC_FAILED: i32 = 0;
const RAPIDLZ_ACCELERATION_MAX: i32 = 0;
const RAPIDLZ_MIN_HASH_BIT: u32 = 0;
const RAPIDLZ_MAX_HASH_BIT: u32 = 0;
const RAPIDLZ_SRC_SIZE_THRESHOLD: usize = 0;
const RAPIDLZ_HASH_TYPE_4: u32 = 0;
const RAPIDLZ_HASH_TYPE_5: u32 = 0;

// Assuming these functions are defined elsewhere
fn RAPIDLZ_LOG(code: i32, message: &str) {}
fn RapidlzHighBit64(size: usize) -> u32 { 0 }
fn RapidlzCompressBound(src_size: usize) -> usize { 0 }
fn RapidlzCompressProcess(dst: *mut c_void, dst_size: usize, src: *const c_void, src_size: usize, c_ctx: *mut RapidlzCCtx) -> usize { 0 }
fn RapidlzCCtxFree(c_ctx: *mut RapidlzCCtx) {}

#[repr(C)]
struct RapidlzCCtx {
    hash_bits: u32,
    hash_type: u32,
    hash_table: *mut u8,
    step: u8,
    buffer_limit: bool,
}

pub fn rapidlz_compress(
    src: *const c_void,
    dst: *mut c_void,
    src_size: usize,
    dst_size: usize,
    acceleration: c_int,
) -> usize {
    if src.is_null() || dst.is_null() || src_size == 0 || dst_size == 0 {
        RAPIDLZ_LOG(RAPIDLZ_INPUT_INVALID, "input invalid\n");
        return 0;
    }
    if acceleration < 1 || acceleration > RAPIDLZ_ACCELERATION_MAX {
        RAPIDLZ_LOG(RAPIDLZ_INPUT_INVALID, format!("acceleration:{}\n", acceleration).as_str());
        return 0;
    }

    let c_ctx_layout = Layout::new::<RapidlzCCtx>();
    let c_ctx = unsafe { alloc(c_ctx_layout) as *mut RapidlzCCtx };
    if c_ctx.is_null() {
        RAPIDLZ_LOG(RAPIDLZ_MALLOC_FAILED, "cCtx malloc failed\n");
        return 0;
    }

    unsafe {
        (*c_ctx).hash_bits = RAPIDLZ_MIN_HASH_BIT;
    }

    let total_hash_size = if src_size <= RAPIDLZ_SRC_SIZE_THRESHOLD {
        unsafe {
            (*c_ctx).hash_type = RAPIDLZ_HASH_TYPE_4;
            if src_size >= 64 {
                (*c_ctx).hash_bits = if RapidlzHighBit64(src_size) > RAPIDLZ_MAX_HASH_BIT {
                    RAPIDLZ_MAX_HASH_BIT + 1
                } else {
                    RapidlzHighBit64(src_size)
                };
            }
        }
        size_of::<u16>() * (1 << unsafe { (*c_ctx).hash_bits }) as usize
    } else {
        unsafe {
            (*c_ctx).hash_type = RAPIDLZ_HASH_TYPE_5;
            (*c_ctx).hash_bits = RAPIDLZ_MAX_HASH_BIT;
        }
        size_of::<u32>() * (1 << unsafe { (*c_ctx).hash_bits }) as usize
    };

    let table_layout = Layout::array::<u8>(total_hash_size).unwrap();
    let table = unsafe { alloc(table_layout) };
    if table.is_null() {
        RAPIDLZ_LOG(RAPIDLZ_MALLOC_FAILED, "hash table malloc failed\n");
        unsafe { dealloc(c_ctx as *mut u8, c_ctx_layout) };
        return 0;
    }

    unsafe {
        std::ptr::write_bytes(table, 0, total_hash_size);
        (*c_ctx).hash_table = table;
        (*c_ctx).step = acceleration as u8;
        (*c_ctx).buffer_limit = dst_size < RapidlzCompressBound(src_size);
    }

    let c_size = RapidlzCompressProcess(dst, dst_size, src, src_size, c_ctx);
    RapidlzCCtxFree(c_ctx);
    c_size
}
