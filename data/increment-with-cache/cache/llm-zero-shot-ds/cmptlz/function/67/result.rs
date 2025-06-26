pub fn cmpt_price_literal(
    enc_ctx: &mut CmptLzEncCtx,
    match_mode: bool,
    match_byte: u32,
    symbol: u32,
) -> u32 {
    let pos = enc_ctx.lit_marcov.pos;
    let prev_byte = enc_ctx.lit_marcov.prev_byte;
    let lit_ctx = enc_ctx.lit_marcov.lc_bits;
    let lp_mask = enc_ctx.lit_marcov.pos_mask;
    let sub_coder = cmpt_lit_subcoder(
        &mut enc_ctx.lit_marcov.literal,
        lit_ctx,
        lp_mask,
        pos,
        prev_byte,
    );

    let mut price = 0;
    if !match_mode {
        price = cmpt_price_symbol(enc_ctx, sub_coder, 8, symbol);
    } else {
        let mut offset = 0x100;
        let mut symbol = symbol + (1 << 8);
        let mut match_byte = match_byte;
        loop {
            match_byte <<= 1;
            let match_bit = match_byte & offset;
            let sub_coder_index = offset + match_bit + (symbol >> 8);
            let bit = (symbol >> 7) & 1;
            price += cmpt_price_one_bit(enc_ctx, sub_coder[sub_coder_index as usize], bit);
            symbol <<= 1;
            offset &= !(match_byte ^ symbol);
            if symbol >= (1 << 16) {
                break;
            }
        }
    }
    price
}
