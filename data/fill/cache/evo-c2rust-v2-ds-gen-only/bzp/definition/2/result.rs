#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct InDeComdata {
    pub input: Ptr<BzpStream>,
    pub output: Ptr<BzpStream>,
    pub lasChar: i32,
    pub num: i32,
    pub buf: u32,
    pub nBuf: i32,
    pub blockSize: i32,
    pub blockCRC: u32,
    pub list: Array<i32, { BZP_ASCII_SIZE!() }>,
}
