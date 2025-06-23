#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct BzpMtfInfo {
    pub block: Ptr<u8>,
    pub map: Ptr<i32>,
    pub mtfV: Ptr<i32>,
    pub inUse: Ptr<bool>,
    pub mtfFreq: Array<i32, { BZP_MAX_ALPHA_SIZE!() }>,
    pub nBlock: i32,
    pub nMtf: i32,
    pub nUse: i32,
    pub pad: i32,
}
