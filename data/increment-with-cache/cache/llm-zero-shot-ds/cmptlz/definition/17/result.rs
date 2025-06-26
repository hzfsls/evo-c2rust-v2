#[repr(C)]
pub struct CmptRcCtx {
    range: u32,
    cache: u64,
    low: u64,
    cache_size: u64,
    buf: *mut u8,
    buf_base: *mut u8,
    out_buf: *mut u8,
    out_buf_left: usize,
}
