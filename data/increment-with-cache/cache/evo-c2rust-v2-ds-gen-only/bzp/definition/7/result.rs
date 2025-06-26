#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct BzpBwtDecodeInfo {
    pub sorted: Ptr<i32>,
    pub block: Ptr<u8>,
    pub deCode: Ptr<u8>,
    pub nBlock: i32,
    pub oriPtr: i32,
}
