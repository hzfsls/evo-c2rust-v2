pub fn cmpt_price_gen_dist_table(enc_ctx: &mut CmptLzEncCtx) {
    let mut dist_state = 0;

    loop {
        let tmp_price_dist_slot = &mut enc_ctx.price_dist_slot_table[dist_state];

        for i in 0..enc_ctx.dist_table_size {
            tmp_price_dist_slot[i] = cmpt_price_symbol(
                enc_ctx,
                enc_ctx.prob_dist_slot[dist_state],
                CMPTLZ_DIST_SLOT_BITS,
                i,
            );
        }

        for i in 14..enc_ctx.dist_table_size {
            tmp_price_dist_slot[i] += cmpt_price_one_bit_direct((i >> 1) - 1 - CMPTLZ_ALIGN_BITS);
        }

        for i in 0..4 {
            enc_ctx.price_dist_table[dist_state][i] = tmp_price_dist_slot[i];
        }

        dist_state += 1;
        if dist_state >= CMPTLZ_DIST_STATE_TOTAL {
            break;
        }
    }

    for i in 4..128 {
        let dist_slot = pos_sloter(i);
        let footer_bits = (dist_slot >> 1) - 1;
        let base = (2 | (dist_slot & 1)) << footer_bits;
        let price = cmpt_price_symbol_reverse(
            enc_ctx,
            enc_ctx.prob_dist_special + base - dist_slot - 1,
            footer_bits,
            i - base,
        );

        for dist_state in 0..4 {
            enc_ctx.price_dist_table[dist_state][i] =
                price + enc_ctx.price_dist_slot_table[dist_state][dist_slot];
        }
    }

    enc_ctx.match_price_count = 0;
}
