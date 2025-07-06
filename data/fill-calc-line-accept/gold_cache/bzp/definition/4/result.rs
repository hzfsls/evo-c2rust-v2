#[derive(Default)]
#[repr(C)]
pub struct BzpOutComdata {
    pub out: Ptr<u8>,
    pub num: i32,
    pub buf: u32,
    pub nBuf: i32,
    pub blockSize: i32,
}