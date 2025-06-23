pub struct BzpBwtDecodeInfo {
    pub sorted: *mut i32,
    pub block: *mut u8,
    pub deCode: *mut u8,
    pub nBlock: i32,
    pub oriPtr: i32,
}
