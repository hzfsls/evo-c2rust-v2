#[repr(C)]
#[derive(Default)]
pub struct TagMd5Ctx {
    pub aulState: Array<u32, 4>,
    pub aulCount: Array<u32, 2>,
    pub aucBuffer: Array<u8, 64>,
    pub uiPos: u32,
}

pub type MD5_CTX = TagMd5Ctx;
