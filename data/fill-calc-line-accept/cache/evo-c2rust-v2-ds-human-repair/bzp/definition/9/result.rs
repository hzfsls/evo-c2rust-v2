#[repr(C)]
#[derive(Default)]
pub struct BzpHuffmanInfo {
    pub heap: Array<i32, { BZP_MAX_ALPHA_SIZE!() + 1 }>,
    pub weight: Array<i32, { BZP_MAX_ALPHA_SIZE!() * 2 }>,
    pub parent: Array<i32, { BZP_MAX_ALPHA_SIZE!() * 2 }>,
    pub len: Array<i32, { BZP_MAX_ALPHA_SIZE!() }>,
    pub table: Array<i32, { BZP_MAX_ALPHA_SIZE!() }>,
    pub nHeap: i32,
    pub nWeight: i32,
    pub alphaSize: i32,
}
