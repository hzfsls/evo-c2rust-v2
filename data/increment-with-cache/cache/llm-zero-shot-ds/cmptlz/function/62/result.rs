fn cmptlz_enc_normal_match(
    enc_ctx: &mut CmptLzEncCtx,
    nowpos32: u32,
    back_res: u32,
    len_res: u32,
) -> Result<(), CmptError> {
    let pos_state = nowpos32 & enc_ctx.pb_mask;
    let mut range = enc_ctx.rc_ctx.range;

    // Process isMatch bit
    let probs = &mut enc_ctx.is_match[enc_ctx.state as usize][pos_state as usize];
    let (bit0_prob, new_bound) = cmpt_rc_get_newbound(probs, range)?;
    cmpt_rc_bit_1_process(&mut enc_ctx.rc_ctx, probs, new_bound, &mut range, bit0_prob)?;

    // Process isRep bit
    let probs = &mut enc_ctx.is_rep[enc_ctx.state as usize];
    let (bit0_prob, new_bound) = cmpt_rc_get_newbound(probs, range)?;
    cmpt_rc_bit_0_process(&mut enc_ctx.rc_ctx, probs, new_bound, &mut range, bit0_prob)?;

    enc_ctx.rc_ctx.range = range;
    let state = enc_ctx.state;
    enc_ctx.state = cmpt_state_update_when_match(state);

    // Process match length
    cmpt_rc_len_process(&mut enc_ctx.match_len_encoder, &mut enc_ctx.rc_ctx, len_res, pos_state)?;

    // Update reps
    let back_res = back_res - CMPTLZ_NUM_REPS;
    enc_ctx.reps[3] = enc_ctx.reps[2];
    enc_ctx.reps[2] = enc_ctx.reps[1];
    enc_ctx.reps[1] = enc_ctx.reps[0];
    enc_ctx.reps[0] = back_res;

    enc_ctx.match_price_count += 1;

    // Process position slot
    let pos_slot = pos_sloter(back_res);
    cmpt_rc_pos_slot_process(enc_ctx, pos_slot, len_res)?;
    
    if back_res >= 4 {
        cmpt_rc_dist_process(enc_ctx, pos_slot, back_res)?;
    }

    Ok(())
}
