fn cmpt_price_long_rep(enc_ctx: &CmptLzEncCtx, long_rep_index: u32, state: CmptlzState, pos_state: u32) -> u32 {
    let mut price = 0;
    match long_rep_index {
        0 => {
            price = cmpt_price_bit0(enc_ctx, enc_ctx.is_rep_g0[state as usize]) 
                  + cmpt_price_bit1(enc_ctx, enc_ctx.is_rep0_long[state as usize][pos_state as usize]);
        }
        1 => {
            price = cmpt_price_bit1(enc_ctx, enc_ctx.is_rep_g0[state as usize]) 
                  + cmpt_price_bit0(enc_ctx, enc_ctx.is_rep_g1[state as usize]);
        }
        2 => {
            price = cmpt_price_bit1(enc_ctx, enc_ctx.is_rep_g0[state as usize]) 
                  + cmpt_price_bit1(enc_ctx, enc_ctx.is_rep_g1[state as usize]) 
                  + cmpt_price_bit0(enc_ctx, enc_ctx.is_rep_g2[state as usize]);
        }
        3 => {
            price = cmpt_price_bit1(enc_ctx, enc_ctx.is_rep_g0[state as usize]) 
                  + cmpt_price_bit1(enc_ctx, enc_ctx.is_rep_g1[state as usize]) 
                  + cmpt_price_bit1(enc_ctx, enc_ctx.is_rep_g2[state as usize]);
        }
        _ => {}
    }
    price
}
