pub fn cmpt_rc_len_process(
    len_encoder: &mut CmptLenEncoder,
    rc_ctx: &mut CmptRcCtx,
    len: u32,
    pos_state: u64,
) -> Result<(), CmptError> {
    let mut range = rc_ctx.range;
    let mut len = len - CMPTLZ_MATCH_LEN_MIN;

    let mut probs = &mut len_encoder.low;
    let (bit0_prob, new_bound) = cmpt_rc_get_newbound(probs, range);
    if len >= CMPT_LEN_BOUND {
        cmpt_rc_bit_1_process(rc_ctx, probs, new_bound, &mut range, bit0_prob)?;
        probs = &mut len_encoder.low[CMPT_LEN_BOUND..];
        let (bit0_prob, new_bound) = cmpt_rc_get_newbound(probs, range);
        if len >= CMPT_LEN_BOUND * CMPT_DOUBLE {
            cmpt_rc_bit_1_process(rc_ctx, probs, new_bound, &mut range, bit0_prob)?;
            rc_ctx.range = range;
            cmpt_rc_lit_process(rc_ctx, &mut len_encoder.high, len - CMPT_LEN_BOUND * CMPT_DOUBLE)?;
            return Ok(());
        }
        len -= CMPT_LEN_BOUND;
    }

    cmpt_rc_bit_0_process(rc_ctx, probs, new_bound, &mut range, bit0_prob)?;
    probs = &mut len_encoder.low[(pos_state << (1 + 3))..];
    let mut bit = len >> 2;
    cmpt_rc_bit_process(rc_ctx, &mut probs[1], bit, bit0_prob, &mut range, new_bound)?;
    let mut m = (1 << 1) + bit;
    bit = (len >> 1) & 1;
    cmpt_rc_bit_process(rc_ctx, &mut probs[m as usize], bit, bit0_prob, &mut range, new_bound)?;
    m = (m << 1) + bit;
    bit = len & 1;
    cmpt_rc_bit_process(rc_ctx, &mut probs[m as usize], bit, bit0_prob, &mut range, new_bound)?;
    rc_ctx.range = range;
    Ok(())
}
