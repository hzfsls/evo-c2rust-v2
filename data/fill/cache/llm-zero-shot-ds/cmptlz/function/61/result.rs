use std::u32;

fn cmptlz_dp_try_cur_and_lit(
    enc_ctx: &mut CmptLzEncCtx,
    cur_price: u32,
    cur_state: CmptlzState,
    pos_state: u32,
    cur: u32,
    latest_match_byte: u8,
    cur_byte: u8,
) {
    let is_literal_state = cur_state < 7;
    let is_match_mode = !is_literal_state;
    let cur_and_lit_price = cur_price 
        + cmpt_price_bit0(enc_ctx, enc_ctx.is_match[cur_state as usize][pos_state as usize])
        + cmpt_price_literal(enc_ctx, is_match_mode, latest_match_byte, cur_byte);
    
    if cur_and_lit_price < enc_ctx.opts[(cur + 1) as usize].price {
        enc_ctx.opts[(cur + 1) as usize].price = cur_and_lit_price;
        enc_ctx.opts[(cur + 1) as usize].pos_prev = cur;
        enc_ctx.opts[(cur + 1) as usize].back_prev = u32::MAX;
    }
}
