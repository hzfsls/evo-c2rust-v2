pub fn cmpt_price_one_bit(enc_ctx: &CmptLzEncCtx, bit0_prob: CmptlzProb, curbit: u32) -> u32 {
    let mask = if curbit == 0 { 0 } else { CMPTLZ_PROB_MAX_NUM - 1 };
    let index = (bit0_prob ^ (mask & (CMPTLZ_PROB_MAX_NUM - 1))) >> CMPT_PRICE_BITS_MOVING_NUM;
    enc_ctx.price_root_table[index as usize]
}
