#[inline]
fn cmptlz_dp_try_cur_and_short(
    enc_ctx: &mut CmptLzEncCtx,
    rep_match_price: u32,
    cur: u32,
    cur_state: CmptlzState,
    pos_state: u32,
) {
    let short_rep_price = rep_match_price + cmpt_price_short_rep(enc_ctx, cur_state, pos_state);
    if short_rep_price < enc_ctx.opts[(cur + 1) as usize].price {
        enc_ctx.opts[(cur + 1) as usize].price = short_rep_price;
        enc_ctx.opts[(cur + 1) as usize].pos_prev = cur;
        enc_ctx.opts[(cur + 1) as usize].back_prev = 0;
    }
}
