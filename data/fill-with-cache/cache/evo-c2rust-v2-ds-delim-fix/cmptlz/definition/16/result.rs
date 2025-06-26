#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct CmptlzEncParam {
    pub level: i32,
    pub dictSize: u32,
    pub litCtx: i32,
    pub litPos: i32,
    pub posBits: i32,
    pub fastBytes: i32,
    pub numThreads: i32,
}
