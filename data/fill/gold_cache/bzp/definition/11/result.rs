#[derive(Default)]
#[repr(C)]
pub struct BzpBwtDecodeInfo {
    pub sorted: Ptr<i32>,
    pub block: Ptr<u8>,
    pub deCode: Ptr<u8>,
    pub nBlock: i32,
    pub oriPtr: i32,
}