pub fn cmpt_rc_len_process(
    len_encoder: &mut CmptLenEncoder,
    rc_ctx: &mut CmptRcCtx,
    len: u32,
    pos_state: u64,
) -> i32 {
    let mut shift_res = CMPT_OK;
    let mut range = rc_ctx.range;
    let mut new_bound;
    let mut bit0_prob;
    
    let len = len - CMPTLZ_MATCH_LEN_MIN;
    let mut probs = &mut len_encoder.low;
    
    // CMPT_RC_GET_NEWBOUND macro
    bit0_prob = *probs;
    new_bound = (range >> CMPT_RC_BIT_MODEL_TOTAL_BITS) * bit0_prob as u32;
    
    if len >= CMPT_LEN_BOUND {
        // CMPT_RC_BIT_1_PROCESS macro
        rc_ctx.low = rc_ctx.low.wrapping_add(new_bound);
        range -= new_bound;
        *probs = bit0_prob + ((CMPT_RC_BIT_MODEL_TOTAL - bit0_prob) >> CMPT_RC_NUM_MOVE_BITS);
        shift_res = cmpt_rc_normalize(rc_ctx, range);
        if shift_res != CMPT_OK {
            return shift_res;
        }
        
        probs = &mut len_encoder.low[CMPT_LEN_BOUND..];
        
        // CMPT_RC_GET_NEWBOUND again
        bit0_prob = *probs;
        new_bound = (range >> CMPT_RC_BIT_MODEL_TOTAL_BITS) * bit0_prob as u32;
        
        if len >= CMPT_LEN_BOUND * CMPT_DOUBLE {
            // CMPT_RC_BIT_1_PROCESS again
            rc_ctx.low = rc_ctx.low.wrapping_add(new_bound);
            range -= new_bound;
            *probs = bit0_prob + ((CMPT_RC_BIT_MODEL_TOTAL - bit0_prob) >> CMPT_RC_NUM_MOVE_BITS);
            shift_res = cmpt_rc_normalize(rc_ctx, range);
            if shift_res != CMPT_OK {
                return shift_res;
            }
            
            rc_ctx.range = range;
            shift_res = cmpt_rc_lit_process(rc_ctx, &mut len_encoder.high, len - CMPT_LEN_BOUND * CMPT_DOUBLE);
            if shift_res != CMPT_OK {
                return shift_res;
            }
            return CMPT_OK;
        }
        
        let len = len - CMPT_LEN_BOUND;
    }
    
    // CMPT_RC_BIT_0_PROCESS macro
    range = new_bound;
    *probs = bit0_prob - (bit0_prob >> CMPT_RC_NUM_MOVE_BITS);
    shift_res = cmpt_rc_normalize(rc_ctx, range);
    if shift_res != CMPT_OK {
        return shift_res;
    }
    
    probs = &mut len_encoder.low[(pos_state << (1 + 3))..];
    
    let mut bit = len >> 2;
    // CMPT_RC_BIT_PROCESS macro
    {
        let prob = &mut probs[1];
        if bit != 0 {
            rc_ctx.low = rc_ctx.low.wrapping_add(new_bound);
            range -= new_bound;
            *prob = bit0_prob + ((CMPT_RC_BIT_MODEL_TOTAL - bit0_prob) >> CMPT_RC_NUM_MOVE_BITS);
        } else {
            range = new_bound;
            *prob = bit0_prob - (bit0_prob >> CMPT_RC_NUM_MOVE_BITS);
        }
        shift_res = cmpt_rc_normalize(rc_ctx, range);
        if shift_res != CMPT_OK {
            return shift_res;
        }
    }
    
    let mut m = (1 << 1) + bit;
    bit = (len >> 1) & 1;
    // CMPT_RC_BIT_PROCESS again
    {
        let prob = &mut probs[m];
        if bit != 0 {
            rc_ctx.low = rc_ctx.low.wrapping_add(new_bound);
            range -= new_bound;
            *prob = bit0_prob + ((CMPT_RC_BIT_MODEL_TOTAL - bit0_prob) >> CMPT_RC_NUM_MOVE_BITS);
        } else {
            range = new_bound;
            *prob = bit0_prob - (bit0_prob >> CMPT_RC_NUM_MOVE_BITS);
        }
        shift_res = cmpt_rc_normalize(rc_ctx, range);
        if shift_res != CMPT_OK {
            return shift_res;
        }
    }
    
    m = (m << 1) + bit;
    bit = len & 1;
    // CMPT_RC_BIT_PROCESS again
    {
        let prob = &mut probs[m];
        if bit != 0 {
            rc_ctx.low = rc_ctx.low.wrapping_add(new_bound);
            range -= new_bound;
            *prob = bit0_prob + ((CMPT_RC_BIT_MODEL_TOTAL - bit0_prob) >> CMPT_RC_NUM_MOVE_BITS);
        } else {
            range = new_bound;
            *prob = bit0_prob - (bit0_prob >> CMPT_RC_NUM_MOVE_BITS);
        }
        shift_res = cmpt_rc_normalize(rc_ctx, range);
        if shift_res != CMPT_OK {
            return shift_res;
        }
    }
    
    rc_ctx.range = range;
    CMPT_OK
}
