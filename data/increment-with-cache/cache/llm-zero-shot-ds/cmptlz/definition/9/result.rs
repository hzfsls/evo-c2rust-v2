#[repr(C)]
struct CmptLzDecProt {
    lit_ctx: u8,
    lit_pos: u8,
    pos_bits: u8,
    align_pad: u8,
    dic_size: u32,
}
