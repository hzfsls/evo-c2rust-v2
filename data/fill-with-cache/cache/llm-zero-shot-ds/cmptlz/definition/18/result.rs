#[repr(C)]
pub struct CmptRcCtx {
    range: u32,
    cache: u64,
    low: u64,
    cacheSize: u64,
    buf: *mut u8,
    bufBase: *mut u8,
    outBuf: *mut u8,
    outBufLeft: usize,
}
