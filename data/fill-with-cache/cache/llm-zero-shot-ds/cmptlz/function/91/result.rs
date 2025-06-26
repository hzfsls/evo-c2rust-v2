fn cmpt_price_bit1(enc_ctx: &CmptLzEncCtx, bit0_prob: CmptlzProb) -> u32 {
    enc_ctx.price_root_table[((bit0_prob ^ (CMPTLZ_PROB_MAX_NUM - 1)) >> CMPT_PRICE_BITS_MOVING_NUM) as usize]
}
