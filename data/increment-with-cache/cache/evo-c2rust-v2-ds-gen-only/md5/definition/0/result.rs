#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct MD5_CTX {
    pub aulState: Array<u32, 4>,
    pub aulCount: Array<u32, 2>,
    pub aucBuffer: Array<u8, 64>,
    pub uiPos: u32,
}
