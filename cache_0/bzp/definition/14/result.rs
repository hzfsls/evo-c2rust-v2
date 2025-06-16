#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct BzpStream {
    pub filePtr: FilePtr,
    pub nBuf: i32,
    pub pos: i32,
    pub buf: Array<u8, { BZP_BUF_SIZE!() }>,
}
