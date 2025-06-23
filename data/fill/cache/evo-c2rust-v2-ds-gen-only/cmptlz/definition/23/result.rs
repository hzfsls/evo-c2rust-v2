#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct CmptlzMatchPair {
    pub len: u32,
    pub dist: u32,
}
