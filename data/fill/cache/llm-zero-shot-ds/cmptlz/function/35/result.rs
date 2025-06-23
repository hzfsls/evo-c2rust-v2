use std::ptr;

static fn cmpt_lz_match_dec(
    dec_ctx: &mut CmptLzDecCtx,
    p_range: &mut u32,
    p_range_code: &mut u32,
    p_range_bound: &mut u32,
    dic_pos_limit: usize,
    pos_state: u32,
) -> u32 {
    let probs_matrix = cmpt_lz_get_probs_matrix(dec_ctx);
    let prob_slot = cmpt_lz_get_match_len_coder_prob(probs_matrix);
    let match_len = cmpt_lz_len_dec(
        dec_ctx,
        prob_slot,
        p_range,
        p_range_code,
        p_range_bound,
        pos_state,
    );

    let match_dist = cmpt_lz_dist_dec(
        dec_ctx,
        probs_matrix,
        p_range,
        p_range_code,
        p_range_bound,
        match_len,
    );

    if match_dist > dec_ctx.dict_buf_size {
        if match_dist == 0xFFFFFFFF {
            dec_ctx.remain_len = CMPTLZ_MATCH_MAX_LEN;
            dec_ctx.state -= CMPTLZ_MKSTATE_NUM;
            return CMPT_OK;
        } else {
            return CMPT_ERROR_DATA;
        }
    }
    cmpt_lz_dec_by_dist_and_len(dec_ctx, match_dist, match_len + 2, dic_pos_limit)
}
