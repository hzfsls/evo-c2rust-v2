#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct CmptRcCtx {
    pub range: u32,
    pub cache: u64,
    pub low: u64,
    pub cacheSize: u64,
    pub buf: Ptr<u8>,
    pub bufBase: Ptr<u8>,
    pub outBuf: Ptr<u8>,
    pub outBufLeft: usize,
}
