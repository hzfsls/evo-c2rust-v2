pub struct LitMarcov {
    pos: u32,
    prev_byte: u32,
    literal: [[CmptlzProb; CMPTLZ_LIT_MAX_SIZE]; 1 << CMPTLZ_LCLP_MAX],
    lc_bits: u32,
    pos_mask: u32,
}
