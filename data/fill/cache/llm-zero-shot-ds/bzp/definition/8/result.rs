#[repr(C)]
pub struct BzpMtfInfo {
    pub block: *mut u8,
    pub map: *mut i32,
    pub mtfV: *mut i32,
    pub inUse: *mut bool,
    pub mtfFreq: [i32; BZP_MAX_ALPHA_SIZE],
    pub nBlock: i32,
    pub nMtf: i32,
    pub nUse: i32,
    pub pad: i32,
}
