pub fn cmpt_lz_try_dec_one_packet(
    dec_ctx: &mut CmptLzDecCtx,
    buf_try_dec: &[u8],
    pbuf_limit: &mut *const u8,
) -> i32 {
    let mut range_bound = 0;
    let mut range = dec_ctx.range;
    let mut range_code = dec_ctx.code;
    let mut mk_state = dec_ctx.state;
    let buf_limit = *pbuf_limit;
    let probs_matrix = cmpt_lz_get_probs_matrix(dec_ctx);
    let pb_mask = (1u32 << dec_ctx.prop.pos_bits) - 1;
    let pos_state = CMPTLZ_CALC_POS_STATE(dec_ctx.processed_pos, pb_mask);
    let prob_slot1 = cmpt_lz_get_is_match_prob(probs_matrix) + pos_state + mk_state;
    range_bound = (range >> CMPTLZ_PROB_LG_BIT) * (*prob_slot1);
    
    if range_code < range_bound {
        CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT0!(range, range_bound);
        CMPTLZ_RANGE_TRY_NORMALIZE!(range, range_code, buf_try_dec, buf_limit);
        return cmpt_lz_try_dec_lit_packet(dec_ctx, range, range_code, range_bound, buf_try_dec, pbuf_limit);
    }
    
    CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT1!(range, range_code, range_bound);
    CMPTLZ_RANGE_TRY_NORMALIZE!(range, range_code, buf_try_dec, buf_limit);
    
    let prob_slot2 = cmpt_lz_get_is_rep_prob(probs_matrix) + mk_state;
    range_bound = (range >> CMPTLZ_PROB_LG_BIT) * (*prob_slot2);
    
    let prob_slot;
    if range_code < range_bound {
        CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT0!(range, range_bound);
        prob_slot = cmpt_lz_get_match_len_coder_prob(probs_matrix);
        mk_state = 0;
    } else {
        if dec_ctx.dict_pos >= dec_ctx.dict_buf_size {
            return CMPT_ERROR_DATA;
        }
        CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT1!(range, range_code, range_bound);
        CMPTLZ_RANGE_TRY_NORMALIZE!(range, range_code, buf_try_dec, buf_limit);
        
        let prob_slot3 = cmpt_lz_get_is_rep_g0_prob(probs_matrix) + mk_state;
        range_bound = (range >> CMPTLZ_PROB_LG_BIT) * (*prob_slot3);
        
        if range_code < range_bound {
            CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT0!(range, range_bound);
            CMPTLZ_RANGE_TRY_NORMALIZE!(range, range_code, buf_try_dec, buf_limit);
            
            let prob_slot4 = cmpt_lz_get_is_rep_g0_long_prob(probs_matrix) + pos_state + mk_state;
            range_bound = (range >> CMPTLZ_PROB_LG_BIT) * (*prob_slot4);
            
            if range_code < range_bound {
                CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT0!(range, range_bound);
                CMPTLZ_RANGE_TRY_NORMALIZE!(range, range_code, buf_try_dec, buf_limit);
                *pbuf_limit = buf_try_dec.as_ptr();
                return CMPT_OK;
            } else {
                CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT1!(range, range_code, range_bound);
            }
        } else {
            CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT1!(range, range_code, range_bound);
            CMPTLZ_RANGE_TRY_NORMALIZE!(range, range_code, buf_try_dec, buf_limit);
            
            let prob_slot5 = cmpt_lz_get_is_rep_g1_prob(probs_matrix) + mk_state;
            range_bound = (range >> CMPTLZ_PROB_LG_BIT) * (*prob_slot5);
            
            if range_code < range_bound {
                CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT0!(range, range_bound);
            } else {
                CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT1!(range, range_code, range_bound);
                CMPTLZ_RANGE_TRY_NORMALIZE!(range, range_code, buf_try_dec, buf_limit);
                
                let prob_slot6 = cmpt_lz_get_is_rep_g2_prob(probs_matrix) + mk_state;
                range_bound = (range >> CMPTLZ_PROB_LG_BIT) * (*prob_slot6);
                
                if range_code < range_bound {
                    CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT0!(range, range_bound);
                } else {
                    CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT1!(range, range_code, range_bound);
                }
            }
        }
        prob_slot = cmpt_lz_get_rep_len_coder_prob(probs_matrix);
        mk_state = CMPTLZ_MKSTATE_NUM;
    }
    
    cmpt_lz_try_dec_len_and_dist(
        dec_ctx,
        mk_state,
        range,
        range_code,
        range_bound,
        prob_slot,
        buf_try_dec,
        pbuf_limit,
    )
}
