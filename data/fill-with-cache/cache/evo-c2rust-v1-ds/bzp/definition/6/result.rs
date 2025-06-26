#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct BzpBwtInfo {
    pub sortBlock: Ptr<i32>,
    pub idx: Ptr<i32>,
    pub isStartPos: Ptr<i32>,
    pub block: Ptr<u8>,
    pub blockCRC: u32,
    pub combinedCRC: u32,
    pub nBlockMax: i32,
    pub blockId: i32,
    pub nBlock: i32,
    pub oriPtr: i32,
    pub inUse: Array<bool, { BZP_ASCII_SIZE!() }>,
}
