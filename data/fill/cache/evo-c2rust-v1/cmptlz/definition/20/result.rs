#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct TagCmptMatchFinder {
    pub srcStart: Ptr<u8>,
    pub srcLen: usize,
    pub hashRootTable: Array<u32, 256>,
    pub mfStart: u32,
    pub niceLen: u32,
    pub readAhead: u32,
    pub readPos: u32,
    pub cyclePos: u32,
    pub cycleSize: u32,
    pub offset: u32,
    pub hash: Ptr<u32>,
    pub son: Ptr<u32>,
    pub depth: u32,
    pub hashCount: u32,
    pub sonsCount: u32,
    pub hashMask: u32,
}
