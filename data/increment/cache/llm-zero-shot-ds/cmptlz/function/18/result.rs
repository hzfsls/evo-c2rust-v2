#[inline]
fn cmptlz_dp_init_match(
    enc_ctx: &mut CmptLzEncCtx,
    matches_count: u32,
    normal_match_price: u32,
    pos_state: u32,
    mut len: u32,
) {
    let mut i = 0;
    while len > enc_ctx.matches[i].len {
        i += 1;
    }
    loop {
        let dist = enc_ctx.matches[i].dist;
        let cur_and_len_price = normal_match_price + cmpt_price_dist_with_len(enc_ctx, dist, len, pos_state);
        if cur_and_len_price < enc_ctx.opts[len as usize].price {
            enc_ctx.opts[len as usize].price = cur_and_len_price;
            enc_ctx.opts[len as usize].pos_prev = 0;
            enc_ctx.opts[len as usize].back_prev = dist + CMPTLZ_NUM_REPS;
        }
        if len == enc_ctx.matches[i].len {
            i += 1;
            if i == matches_count {
                break;
            }
        }
        len += 1;
    }
}
