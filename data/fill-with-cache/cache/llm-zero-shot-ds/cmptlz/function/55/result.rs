pub fn cmpt_encode_all(enc_ctx: &mut CmptLzEncCtx) -> i32 {
    let rc = enc_ctx.rc_ctx;
    let mf = enc_ctx.mf_ctx;
    
    if mf.src_len == 0 {
        return cmptlz_flush(enc_ctx);
    }
    
    if enc_ctx.nowpos64 == 0 {
        let mut range = rc.range;
        let probs = &mut enc_ctx.is_match[enc_ctx.state][0];
        
        let (bit0_prob, new_bound) = {
            let prob = probs;
            let bound = (range >> CMPT_RC_PROB_BITS) * (prob as u32);
            (prob, bound)
        };
        
        let mut shift_res = CMPT_OK;
        if rc.code < new_bound {
            *probs = bit0_prob - (*probs >> CMPT_RC_MOVE_BITS);
            range = new_bound;
        } else {
            *probs = bit0_prob + ((CMPT_RC_PROB_MAX - bit0_prob) >> CMPT_RC_MOVE_BITS);
            rc.code -= new_bound;
            range -= new_bound;
        }
        
        if range < CMPT_RC_RANGE_MIN {
            shift_res = cmpt_rc_normalize(rc);
            if shift_res != CMPT_OK {
                return shift_res;
            }
        }
        
        rc.range = range;
        
        let cur_byte = *mf.src_start;
        let lit_prob = &mut enc_ctx.lit_marcov.literal[0][0];
        shift_res = cmpt_rc_lit_process(rc, lit_prob, cur_byte);
        if shift_res != CMPT_OK {
            return shift_res;
        }
        
        mf.mf_start += 1;
        enc_ctx.nowpos64 += 1;
        mf.read_pos += 1;
        
        if mf.src_len == 1 {
            return cmptlz_flush(enc_ctx);
        }
    }
    
    loop {
        let res = cmpt_encode_one_block(enc_ctx);
        if res != 0 || enc_ctx.enc_need_finish {
            return res;
        }
    }
}
