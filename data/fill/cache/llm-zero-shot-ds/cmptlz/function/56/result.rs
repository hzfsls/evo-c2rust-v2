fn cmptlz_dp_init_short_rep(enc_ctx: &mut CmptLzEncCtx, rep_match_price: u32, pos_state: u32) {
    let short_rep_price = rep_match_price + cmpt_price_short_rep(enc_ctx, enc_ctx.state, pos_state);
    if short_rep_price < enc_ctx.opts[1].price {
        enc_ctx.opts[1].price = short_rep_price;
        enc_ctx.opts[1].back_prev = 0;
    }
}
