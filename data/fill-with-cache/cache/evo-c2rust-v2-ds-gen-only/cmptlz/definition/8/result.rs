#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct CmptLzDecProt {
    pub litCtx: u8,
    pub litPos: u8,
    pub posBits: u8,
    pub alignPad: u8,
    pub dicSize: u32,
}
