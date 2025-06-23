fn cmpt_price_short_rep(enc_ctx: &CmptLzEncCtx, state: CmptlzState, pos_state: u32) -> u32 {
    cmpt_price_bit0(enc_ctx, enc_ctx.is_rep_g0[state as usize]) 
        + cmpt_price_bit0(enc_ctx, enc_ctx.is_rep0_long[state as usize][pos_state as usize])
}
