#[inline]
fn cmptlz_dp_reverse(enc_ctx: &mut CmptLzEncCtx, cur: u32) {
    enc_ctx.opt_end_index = cur;
    let mut pos_tmp = enc_ctx.opts[cur as usize].pos_prev;
    let mut back_tmp = enc_ctx.opts[cur as usize].back_prev;
    let mut pos_prev;
    let mut back_cur_packet;
    
    loop {
        pos_prev = pos_tmp;
        back_cur_packet = back_tmp;

        back_tmp = enc_ctx.opts[pos_prev as usize].back_prev;
        pos_tmp = enc_ctx.opts[pos_prev as usize].pos_prev;

        enc_ctx.opts[pos_prev as usize].back_prev = back_cur_packet;
        enc_ctx.opts[pos_prev as usize].pos_prev = cur;
        cur = pos_prev;
        
        if cur == 0 {
            break;
        }
    }

    enc_ctx.len_res = enc_ctx.opts[0].pos_prev;
    enc_ctx.back_res = enc_ctx.opts[0].back_prev;
    enc_ctx.opts_cur_index = enc_ctx.opts[0].pos_prev;
}
