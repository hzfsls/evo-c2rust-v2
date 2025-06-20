#[derive(Default)]
#[repr(C)]
pub struct CmptlzMatchPair {
    pub len: u32,
    pub dist: u32,
}