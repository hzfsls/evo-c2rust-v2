pub fn cmpt_price_gen_len_table(enc_ctx: &mut CmptLzEncCtx, len_encoder: &mut CmptLenEncoder) {
    let num_pos_states = 1 << enc_ctx.pos_bits;

    let prob = len_encoder.low[0];
    let a = cmpt_price_bit0(enc_ctx, prob);
    let b = cmpt_price_bit1(enc_ctx, prob);
    let c = b + cmpt_price_bit0(enc_ctx, len_encoder.low[1 << CMPT_LEN_LOW_BITS]);

    for pos_state in 0..num_pos_states {
        let prices = &mut len_encoder.prices[pos_state];
        let probs = &len_encoder.low[pos_state << (1 + CMPT_LEN_LOW_BITS)];
        cmpt_price_set(enc_ctx, probs, a, prices);
        cmpt_price_set(
            enc_ctx,
            &probs[1 << CMPT_LEN_LOW_BITS..],
            c,
            &mut prices[1 << CMPT_LEN_LOW_BITS..],
        );
    }

    let mut i = len_encoder.table_size;
    if i > (1 << CMPT_LEN_LOW_BITS) * CMPT_DOUBLE {
        let probs = &len_encoder.high;
        let prices = &mut len_encoder.prices[0][(1 << CMPT_LEN_LOW_BITS) * CMPT_DOUBLE..];
        i -= (1 << CMPT_LEN_LOW_BITS) * CMPT_DOUBLE - 1;
        i >>= 1;
        let mut b = b + cmpt_price_bit1(enc_ctx, len_encoder.low[1 << CMPT_LEN_LOW_BITS]);

        while i > 0 {
            i -= 1;
            let mut sym = i + (1 << (CMPT_LEN_HIGH_BITS - 1));
            let mut price = b;
            loop {
                let bit = sym & 1;
                sym >>= 1;
                price += cmpt_price_one_bit(enc_ctx, probs[sym as usize], bit);
                if sym < 2 {
                    break;
                }
            }

            let prob = probs[i + (1 << (CMPT_LEN_HIGH_BITS - 1))];
            prices[i * CMPT_DOUBLE] = price + cmpt_price_bit0(enc_ctx, prob);
            prices[i * CMPT_DOUBLE + 1] = price + cmpt_price_bit1(enc_ctx, prob);
        }

        let num = (len_encoder.table_size - (1 << CMPT_LEN_LOW_BITS) * CMPT_DOUBLE)
            * std::mem::size_of_val(&len_encoder.prices[0][0]);

        for pos_state in 1..num_pos_states {
            let src = &len_encoder.prices[0][(1 << CMPT_LEN_LOW_BITS) * CMPT_DOUBLE..];
            let dst = &mut len_encoder.prices[pos_state][(1 << CMPT_LEN_LOW_BITS) * CMPT_DOUBLE..];
            dst[..num].copy_from_slice(&src[..num]);
        }
    }
}
