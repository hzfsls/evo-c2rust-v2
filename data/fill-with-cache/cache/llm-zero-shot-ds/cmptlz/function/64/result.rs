fn cmptlz_dp_try_cur_and_match(
    enc_ctx: &mut CmptLzEncCtx,
    start_len: u32,
    match_count: u32,
    normalmatch_prefix_price: u32,
    cur: u32,
    pos_state: u32,
) {
    let mut i = 0;
    while start_len > enc_ctx.matches[i].len {
        i += 1;
    }
    let mut len_test = start_len;
    loop {
        let cur_back = enc_ctx.matches[i].dist;
        let cur_normalmatch_price = normalmatch_prefix_price
            + cmpt_price_dist_with_len(enc_ctx, cur_back, len_test, pos_state);
        if cur_normalmatch_price < enc_ctx.opts[(cur + len_test) as usize].price {
            enc_ctx.opts[(cur + len_test) as usize].price = cur_normalmatch_price;
            enc_ctx.opts[(cur + len_test) as usize].pos_prev = cur;
            enc_ctx.opts[(cur + len_test) as usize].back_prev = cur_back + CMPTLZ_NUM_REPS;
        }
        if len_test == enc_ctx.matches[i].len {
            i += 1;
            if i == match_count {
                break;
            }
        }
        len_test += 1;
    }
}
