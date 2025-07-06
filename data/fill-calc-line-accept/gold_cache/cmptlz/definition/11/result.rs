#[derive(Default)]
#[repr(C)]
pub struct CmptLzDecOut {
    pub pDestOut: Ptr<u8>,
    pub destOutLen: usize,
    pub destOutFillLen: usize,
}