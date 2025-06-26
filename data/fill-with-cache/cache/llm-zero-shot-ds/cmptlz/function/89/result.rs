fn cmpt_price_one_bit(enc_ctx: &CmptLzEncCtx, bit0_prob: CmptlzProb, curbit: u32) -> u32 {
    enc_ctx.price_root_table[((bit0_prob ^ (0u32.wrapping_sub(curbit) & (CMPTLZ_PROB_MAX_NUM - 1))) >> CMPT_PRICE_BITS_MOVING_NUM) as usize]
}
