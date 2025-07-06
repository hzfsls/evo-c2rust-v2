#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct CmptLzDecIn {
    pub pSrcIn: Ptr<u8>,
    pub strInLen: usize,
    pub strInCostLen: usize,
}
