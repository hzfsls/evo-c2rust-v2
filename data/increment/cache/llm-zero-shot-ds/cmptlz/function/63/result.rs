fn cmptlz_enc_long_rep(
    enc_ctx: &mut CmptLzEncCtx,
    rep_index: u32,
    nowpos32: u32,
    len_res: u32,
) -> Result<(), CmptError> {
    let pos_state = nowpos32 & enc_ctx.pb_mask;
    let mut range = enc_ctx.rc_ctx.range;

    // First isMatch bit
    let probs = &mut enc_ctx.is_match[enc_ctx.state as usize][pos_state as usize];
    let (bit0_prob, new_bound) = cmpt_rc_get_newbound(probs, range);
    cmpt_rc_bit_1_process(enc_ctx.rc_ctx, probs, new_bound, &mut range, bit0_prob)?;

    // isRep bit
    let probs = &mut enc_ctx.is_rep[enc_ctx.state as usize];
    let (bit0_prob, new_bound) = cmpt_rc_get_newbound(probs, range);
    cmpt_rc_bit_1_process(enc_ctx.rc_ctx, probs, new_bound, &mut range, bit0_prob)?;

    // isRepG0 bit
    let probs = &mut enc_ctx.is_rep_g0[enc_ctx.state as usize];
    let (bit0_prob, new_bound) = cmpt_rc_get_newbound(probs, range);

    match rep_index {
        0 => {
            cmpt_rc_bit_0_process(enc_ctx.rc_ctx, probs, new_bound, &mut range, bit0_prob)?;
            
            let probs = &mut enc_ctx.is_rep0_long[enc_ctx.state as usize][pos_state as usize];
            let (bit0_prob, new_bound) = cmpt_rc_get_newbound(probs, range);
            cmpt_rc_bit_1(enc_ctx.rc_ctx, probs, new_bound, &mut range, bit0_prob);
        }
        1 => {
            cmpt_rc_bit_1_process(enc_ctx.rc_ctx, probs, new_bound, &mut range, bit0_prob)?;
            
            let probs = &mut enc_ctx.is_rep_g1[enc_ctx.state as usize];
            let (bit0_prob, new_bound) = cmpt_rc_get_newbound(probs, range);
            cmpt_rc_bit_0(probs, new_bound, &mut range, bit0_prob);

            let real_dist = enc_ctx.reps[1];
            enc_ctx.reps[1] = enc_ctx.reps[0];
            enc_ctx.reps[0] = real_dist;
        }
        2 => {
            cmpt_rc_bit_1_process(enc_ctx.rc_ctx, probs, new_bound, &mut range, bit0_prob)?;
            
            let probs = &mut enc_ctx.is_rep_g1[enc_ctx.state as usize];
            let (bit0_prob, new_bound) = cmpt_rc_get_newbound(probs, range);
            cmpt_rc_bit_1_process(enc_ctx.rc_ctx, probs, new_bound, &mut range, bit0_prob)?;
            
            let probs = &mut enc_ctx.is_rep_g2[enc_ctx.state as usize];
            let (bit0_prob, new_bound) = cmpt_rc_get_newbound(probs, range);
            cmpt_rc_bit_0(probs, new_bound, &mut range, bit0_prob);

            let real_dist = enc_ctx.reps[2];
            enc_ctx.reps[2] = enc_ctx.reps[1];
            enc_ctx.reps[1] = enc_ctx.reps[0];
            enc_ctx.reps[0] = real_dist;
        }
        3 => {
            cmpt_rc_bit_1_process(enc_ctx.rc_ctx, probs, new_bound, &mut range, bit0_prob)?;
            
            let probs = &mut enc_ctx.is_rep_g1[enc_ctx.state as usize];
            let (bit0_prob, new_bound) = cmpt_rc_get_newbound(probs, range);
            cmpt_rc_bit_1_process(enc_ctx.rc_ctx, probs, new_bound, &mut range, bit0_prob)?;
            
            let probs = &mut enc_ctx.is_rep_g2[enc_ctx.state as usize];
            let (bit0_prob, new_bound) = cmpt_rc_get_newbound(probs, range);
            cmpt_rc_bit_1(enc_ctx.rc_ctx, probs, new_bound, &mut range, bit0_prob);

            let real_dist = enc_ctx.reps[3];
            enc_ctx.reps[3] = enc_ctx.reps[2];
            enc_ctx.reps[2] = enc_ctx.reps[1];
            enc_ctx.reps[1] = enc_ctx.reps[0];
            enc_ctx.reps[0] = real_dist;
        }
        _ => {}
    }

    cmpt_rc_normalize(enc_ctx.rc_ctx, &mut range)?;
    enc_ctx.rc_ctx.range = range;

    cmpt_rc_len_process(&mut enc_ctx.rep_len_encoder, enc_ctx.rc_ctx, len_res, pos_state)?;
    enc_ctx.rep_len_price_count -= 1;

    enc_ctx.state = cmpt_state_update_when_longrep(enc_ctx.state);
    Ok(())
}
