pub fn cmpt_price_gen_len_table(enc_ctx: &mut CmptLzEncCtx, len_encoder: &mut CmptLenEncoder) {
    let num_pos_states = 1 << enc_ctx.pos_bits;
    let prob = len_encoder.low[0];
    let b = cmpt_price_bit1(enc_ctx, prob);
    let a = cmpt_price_bit0(enc_ctx, prob);
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
        
        loop {
            i -= 1;
            let sym = i + (1 << (CMPT_LEN_HIGH_BITS - 1));
            let mut price = b;
            let mut temp_sym = sym;
            loop {
                let bit = temp_sym & 1;
                temp_sym >>= 1;
                price += cmpt_price_one_bit(enc_ctx, probs[temp_sym as usize], bit);
                if temp_sym < 2 {
                    break;
                }
            }
            let prob = probs[i as usize + (1 << (CMPT_LEN_HIGH_BITS - 1))];
            prices[i as usize * CMPT_DOUBLE] = price + cmpt_price_bit0(enc_ctx, prob);
            prices[i as usize * CMPT_DOUBLE + 1] = price + cmpt_price_bit1(enc_ctx, prob);
            if i == 0 {
                break;
            }
        }
        
        let num_bytes = (len_encoder.table_size - (1 << CMPT_LEN_LOW_BITS) * CMPT_DOUBLE) 
            * std::mem::size_of_val(&len_encoder.prices[0][0]);
        
        for pos_state in 1..num_pos_states {
            let src = &len_encoder.prices[0][(1 << CMPT_LEN_LOW_BITS) * CMPT_DOUBLE..];
            let dst = &mut len_encoder.prices[pos_state][(1 << CMPT_LEN_LOW_BITS) * CMPT_DOUBLE..];
            dst[..num_bytes / std::mem::size_of_val(&dst[0])].copy_from_slice(
                &src[..num_bytes / std::mem::size_of_val(&src[0])]
            );
        }
    }
}
