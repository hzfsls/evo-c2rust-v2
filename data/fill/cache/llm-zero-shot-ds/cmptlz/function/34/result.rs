fn cmpt_lz_rep_dec(
    dec_ctx: &mut CmptLzDecCtx,
    p_range: &mut u32,
    p_range_code: &mut u32,
    p_range_bound: &mut u32,
    dic_pos_limit: usize,
    pos_state: u32,
) -> u32 {
    let mk_state = dec_ctx.state;
    let mut buf_to_dec = dec_ctx.buf;
    let probs_matrix = cmpt_lz_get_probs_matrix(dec_ctx);
    let mut range = *p_range;
    let mut range_code = *p_range_code;
    let mut range_bound = *p_range_bound;

    let mut prob_slot = cmpt_lz_get_is_rep_g0_prob(probs_matrix) + mk_state;
    if cmpt_lz_is_the_bit_0(prob_slot, range, range_code, range_bound) {
        cmpt_lz_range_update_0(prob_slot, &mut range, &mut range_bound);
        cmpt_lz_range_normalize(&mut range, &mut range_code, &mut buf_to_dec);
        prob_slot = cmpt_lz_get_is_rep_g0_long_prob(probs_matrix) + pos_state + mk_state;
        if cmpt_lz_is_the_bit_0(prob_slot, range, range_code, range_bound) {
            cmpt_lz_range_update_0(prob_slot, &mut range, &mut range_bound);
            cmpt_lz_range_normalize(&mut range, &mut range_code, &mut buf_to_dec);
            *p_range = range;
            *p_range_code = range_code;
            *p_range_bound = range_bound;
            dec_ctx.buf = buf_to_dec;
            cmpt_lz_short_rep_dec(dec_ctx);
            return CMPT_OK;
        } else {
            cmpt_lz_range_update_1(prob_slot, &mut range, &mut range_code, &mut range_bound);
            cmpt_lz_range_normalize(&mut range, &mut range_code, &mut buf_to_dec);
            let rep_dist = dec_ctx.reps[0];
        }
    } else {
        cmpt_lz_range_update_1(prob_slot, &mut range, &mut range_code, &mut range_bound);
        cmpt_lz_range_normalize(&mut range, &mut range_code, &mut buf_to_dec);
        prob_slot = cmpt_lz_get_is_rep_g1_prob(probs_matrix) + mk_state;
        if cmpt_lz_is_the_bit_0(prob_slot, range, range_code, range_bound) {
            cmpt_lz_range_update_0(prob_slot, &mut range, &mut range_bound);
            cmpt_lz_range_normalize(&mut range, &mut range_code, &mut buf_to_dec);
            let rep_dist = dec_ctx.reps[1];
        } else {
            cmpt_lz_range_update_1(prob_slot, &mut range, &mut range_code, &mut range_bound);
            cmpt_lz_range_normalize(&mut range, &mut range_code, &mut buf_to_dec);
            prob_slot = cmpt_lz_get_is_rep_g2_prob(probs_matrix) + mk_state;
            if cmpt_lz_is_the_bit_0(prob_slot, range, range_code, range_bound) {
                cmpt_lz_range_update_0(prob_slot, &mut range, &mut range_bound);
                cmpt_lz_range_normalize(&mut range, &mut range_code, &mut buf_to_dec);
                let rep_dist = dec_ctx.reps[CMPTLZ_REP2];
            } else {
                cmpt_lz_range_update_1(prob_slot, &mut range, &mut range_code, &mut range_bound);
                cmpt_lz_range_normalize(&mut range, &mut range_code, &mut buf_to_dec);
                let rep_dist = dec_ctx.reps[CMPTLZ_REP3];
                dec_ctx.reps[CMPTLZ_REP3] = dec_ctx.reps[CMPTLZ_REP2];
            }
            dec_ctx.reps[CMPTLZ_REP2] = dec_ctx.reps[1];
        }
        dec_ctx.reps[1] = dec_ctx.reps[0];
        dec_ctx.reps[0] = rep_dist;
    }

    *p_range = range;
    *p_range_code = range_code;
    *p_range_bound = range_bound;
    dec_ctx.buf = buf_to_dec;
    dec_ctx.state = if mk_state < CMPTLZ_LIT_STATES { 8 } else { 11 };
    prob_slot = cmpt_lz_get_rep_len_coder_prob(probs_matrix);
    let rep_len = cmpt_lz_len_dec(
        dec_ctx,
        prob_slot,
        p_range,
        p_range_code,
        p_range_bound,
        pos_state,
    );
    cmpt_lz_dec_by_dist_and_len(dec_ctx, rep_dist, rep_len + 2, dic_pos_limit)
}
