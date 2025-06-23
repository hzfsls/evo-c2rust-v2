fn cmptlz_enc_lit(enc_ctx: &mut CmptLzEncCtx, mf: &mut CmptMfCtx, nowpos32: u32) -> Result<(), CmptError> {
    let rc = &mut enc_ctx.rc_ctx;
    let pos_state = nowpos32 & enc_ctx.pb_mask;
    let mut range = rc.range;
    let probs = &mut enc_ctx.is_match[enc_ctx.state as usize][pos_state as usize];
    
    let (bit0_prob, new_bound) = cmpt_rc_get_newbound(probs, range)?;
    let shift_res = cmpt_rc_bit_0_process(rc, probs, new_bound, range, bit0_prob)?;
    if shift_res != CmptError::Ok {
        return Err(shift_res);
    }
    rc.range = range;
    
    let lit_prob = &mut enc_ctx.lit_marcov.literal[0][0];
    let cur_byte = mf.src_start[(mf.read_pos - mf.read_ahead) as usize];
    let probs = cmpt_lit_prob_get(
        enc_ctx, 
        lit_prob, 
        nowpos32, 
        mf.src_start[(mf.read_pos - mf.read_ahead - 1) as usize]
    );
    
    let state = enc_ctx.state;
    cmpt_state_update_when_lit(&mut enc_ctx.state);
    
    if state < 7 {
        let shift_res = cmpt_rc_lit_process(rc, probs, cur_byte)?;
        if shift_res != CmptError::Ok {
            return Err(shift_res);
        }
    } else {
        let match_byte = mf.src_start[(mf.read_pos - enc_ctx.reps[0] - 1 - mf.read_ahead) as usize];
        let shift_res = cmpt_rc_lit_after_match(rc, probs, cur_byte, match_byte)?;
        if shift_res != CmptError::Ok {
            return Err(shift_res);
        }
    }
    
    Ok(())
}
