pub fn cmptlz_dp(enc_ctx: &mut CmptLzEncCtx, mf: &mut CmptMfCtx, position: u32) {
    let cur_index = enc_ctx.opts_cur_index;
    let end_index = enc_ctx.opt_end_index;
    if end_index != cur_index {
        enc_ctx.len_res = enc_ctx.opts[cur_index].pos_prev - cur_index;
        enc_ctx.back_res = enc_ctx.opts[cur_index].back_prev;
        enc_ctx.opts_cur_index = enc_ctx.opts[cur_index].pos_prev;
        return;
    }
    let len_end = cmptlz_dp_init(enc_ctx, mf, position);
    if len_end == u32::MAX {
        return;
    }
    let mut main_reps: [u32; CMPTLZ_NUM_REPS] = enc_ctx.reps;
    let mut cur;
    for cur in 1..len_end {
        enc_ctx.longest_match_len = cmptlz_match_finder(mf, &mut enc_ctx.matches_count, &mut enc_ctx.matches);
        if enc_ctx.longest_match_len >= mf.nice_len {
            break;
        }
        cmptlz_dp_pre(enc_ctx, &main_reps, cur);
        len_end = cmptlz_dp_process(enc_ctx, mf, &main_reps, len_end, position + cur, cur);
    }
    cmptlz_dp_reverse(enc_ctx, cur);
}
