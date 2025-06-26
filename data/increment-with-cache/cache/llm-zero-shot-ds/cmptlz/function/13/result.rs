pub fn cmpt_encode_all(enc_ctx: &mut CmptLzEncCtx) -> i32 {
    let rc = &mut enc_ctx.rc_ctx;
    let mf = &mut enc_ctx.mf_ctx;

    if mf.src_len == 0 {
        return cmptlz_flush(enc_ctx);
    }

    if enc_ctx.nowpos64 == 0 {
        let mut range = rc.range;
        let probs = &mut enc_ctx.is_match[enc_ctx.state as usize][0];
        let (bit0_prob, new_bound) = cmpt_rc_get_newbound(probs, range);
        let mut shift_res = CMPT_OK;
        cmpt_rc_bit_0_process(rc, probs, new_bound, &mut range, bit0_prob, &mut shift_res);
        if shift_res != CMPT_OK {
            return shift_res;
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
