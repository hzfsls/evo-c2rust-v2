fn cmptlz_enc_short_rep(enc_ctx: &mut CmptLzEncCtx, nowpos32: u32) -> Result<(), CmptError> {
    let pos_state = nowpos32 & enc_ctx.pb_mask;
    let mut range = enc_ctx.rc_ctx.range;
    
    // First bit processing
    let probs = &mut enc_ctx.is_match[enc_ctx.state as usize][pos_state as usize];
    let (bit0_prob, new_bound) = cmpt_rc_get_newbound(probs, range);
    cmpt_rc_bit_1_process(&mut enc_ctx.rc_ctx, probs, new_bound, &mut range, bit0_prob)?;
    
    // Second bit processing
    let probs = &mut enc_ctx.is_rep[enc_ctx.state as usize];
    let (bit0_prob, new_bound) = cmpt_rc_get_newbound(probs, range);
    cmpt_rc_bit_1_process(&mut enc_ctx.rc_ctx, probs, new_bound, &mut range, bit0_prob)?;
    
    // Third bit processing
    let probs = &mut enc_ctx.is_rep_g0[enc_ctx.state as usize];
    let (bit0_prob, new_bound) = cmpt_rc_get_newbound(probs, range);
    cmpt_rc_bit_0_process(&mut enc_ctx.rc_ctx, probs, new_bound, &mut range, bit0_prob)?;
    
    // Fourth bit processing
    let probs = &mut enc_ctx.is_rep0_long[enc_ctx.state as usize][pos_state as usize];
    let (bit0_prob, new_bound) = cmpt_rc_get_newbound(probs, range);
    cmpt_rc_bit_0_process(&mut enc_ctx.rc_ctx, probs, new_bound, &mut range, bit0_prob)?;
    
    enc_ctx.rc_ctx.range = range;
    
    // Update state
    enc_ctx.state = cmpt_state_update_when_shortrep(enc_ctx.state);
    
    Ok(())
}
