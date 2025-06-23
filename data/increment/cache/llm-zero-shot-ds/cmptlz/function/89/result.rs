pub fn cmpt_lz_get_num_probs(dec_prot: &CmptLzDecProt) -> u32 {
    const NUM_BASE_PROBS: u32 = /* base value here */;
    NUM_BASE_PROBS + (0x300 << (dec_prot.lit_ctx + dec_prot.lit_pos))
}
