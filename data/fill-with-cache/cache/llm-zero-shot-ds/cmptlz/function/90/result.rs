fn cmpt_price_bit0(enc_ctx: &CmptLzEncCtx, bit0_prob: CmptlzProb) -> u32 {
    enc_ctx.price_root_table[(bit0_prob >> CMPT_PRICE_BITS_MOVING_NUM) as usize]
}
