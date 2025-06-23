fn cmptlz_dp_try_cur_and_long(
    enc_ctx: &mut CmptLzEncCtx,
    prefix_price: u32,
    cur: u32,
    main_rep_index: u32,
    mut len_equal: u32,
    pos_state: u32,
) {
    while len_equal >= CMPTLZ_MATCH_LEN_MIN {
        let cur_long_rep_price = prefix_price + cmpt_price_len(&enc_ctx.rep_len_encoder, len_equal, pos_state);
        let opt_index = cur + len_equal;
        if cur_long_rep_price < enc_ctx.opts[opt_index as usize].price {
            enc_ctx.opts[opt_index as usize].price = cur_long_rep_price;
            enc_ctx.opts[opt_index as usize].pos_prev = cur;
            enc_ctx.opts[opt_index as usize].back_prev = main_rep_index;
        }
        len_equal -= 1;
    }
}
