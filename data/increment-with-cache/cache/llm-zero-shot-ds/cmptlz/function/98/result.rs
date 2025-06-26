pub fn cmpt_lz_dec_direct_process(
    dec_ctx: &mut CmptLzDecCtx,
    dic_pos_limit: usize,
    buf_limit: *const u8,
) -> i32 {
    let pb_mask = ((1u32) << (dec_ctx.prop.pos_bits)) - 1;
    let probs_matrix = cmpt_lz_get_probs_matrix(dec_ctx);

    let mut range = dec_ctx.range;
    let mut range_code = dec_ctx.code;
    let mut range_bound = 0u32;

    loop {
        let proc_pos = dec_ctx.processed_pos;
        let mk_state = dec_ctx.state;
        let pos_state = CMPTLZ_CALC_POS_STATE!(proc_pos, pb_mask);
        let prob_slot = cmpt_lz_get_is_match_prob(probs_matrix) + pos_state + mk_state;

        CMPTLZ_RANGE_NORMALIZE!(range, range_code, dec_ctx.buf);
        if CMPTLZ_IS_THE_BIT_0!(prob_slot, range, range_code, range_bound) {
            CMPTLZ_RANGE_UPDATE_0!(prob_slot, range, range_bound);
            CMPTLZ_RANGE_NORMALIZE!(range, range_code, dec_ctx.buf);
            let dec_res = cmpt_lz_lit_dec(dec_ctx, &mut range, &mut range_code, &mut range_bound);
            if dec_res != CMPT_OK {
                break dec_res as i32;
            }
        } else {
            CMPTLZ_RANGE_UPDATE_1!(prob_slot, range, range_code, range_bound);
            CMPTLZ_RANGE_NORMALIZE!(range, range_code, dec_ctx.buf);

            let prob_slot = cmpt_lz_get_is_rep_prob(probs_matrix) + mk_state;

            if CMPTLZ_IS_THE_BIT_0!(prob_slot, range, range_code, range_bound) {
                CMPTLZ_RANGE_UPDATE_0!(prob_slot, range, range_bound);
                CMPTLZ_RANGE_NORMALIZE!(range, range_code, dec_ctx.buf);
                let dec_res = cmpt_lz_match_dec(
                    dec_ctx,
                    &mut range,
                    &mut range_code,
                    &mut range_bound,
                    dic_pos_limit,
                    pos_state,
                );
                if dec_res != CMPT_OK {
                    break dec_res as i32;
                }
            } else {
                CMPTLZ_RANGE_UPDATE_1!(prob_slot, range, range_code, range_bound);
                CMPTLZ_RANGE_NORMALIZE!(range, range_code, dec_ctx.buf);
                let dec_res = cmpt_lz_rep_dec(
                    dec_ctx,
                    &mut range,
                    &mut range_code,
                    &mut range_bound,
                    dic_pos_limit,
                    pos_state,
                );
                if dec_res != CMPT_OK {
                    break dec_res as i32;
                }
            }
        }

        if dec_ctx.dict_pos >= dic_pos_limit 
            || dec_ctx.buf >= buf_limit 
            || dec_ctx.remain_len >= CMPTLZ_MATCH_MAX_LEN 
        {
            break CMPT_OK as i32;
        }
    }

    dec_ctx.range = range;
    dec_ctx.code = range_code;

    CMPT_OK as i32
}
