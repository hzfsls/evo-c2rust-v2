#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct CmptLzDecOut {
    pub pDestOut: Ptr<u8>,
    pub destOutLen: usize,
    pub destOutFillLen: usize,
}
