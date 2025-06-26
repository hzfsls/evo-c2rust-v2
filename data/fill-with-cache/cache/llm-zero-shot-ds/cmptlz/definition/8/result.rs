#[repr(C)]
pub struct CmptLzDecProt {
    pub lit_ctx: u8,
    pub lit_pos: u8,
    pub pos_bits: u8,
    pub align_pad: u8,
    pub dic_size: u32,
}
