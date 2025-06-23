#[inline]
fn cmptlz_dp_try_cur_and_long(
    enc_ctx: &mut CmptLzEncCtx,
    prefix_price: u32,
    cur: u32,
    main_rep_index: u32,
    mut len_equal: u32,
    pos_state: u32,
) {
    while {
        let cur_long_rep_price = prefix_price + cmpt_price_len(&enc_ctx.rep_len_encoder, len_equal, pos_state);

        if cur_long_rep_price < enc_ctx.opts[(cur + len_equal) as usize].price {
            enc_ctx.opts[(cur + len_equal) as usize].price = cur_long_rep_price;
            enc_ctx.opts[(cur + len_equal) as usize].pos_prev = cur;
            enc_ctx.opts[(cur + len_equal) as usize].back_prev = main_rep_index;
        }

        len_equal -= 1;
        len_equal >= CMPTLZ_MATCH_LEN_MIN
    } {}
}
