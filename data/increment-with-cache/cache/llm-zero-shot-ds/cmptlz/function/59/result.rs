fn cmpt_price_symbol_reverse(enc_ctx: &mut CmptLzEncCtx, symbol_probs: &[CmptlzProb], symbol_bits_num: u32, symbol: u32) -> u32 {
    let mut price = 0;
    let mut i = 1;
    let mut remaining_bits = symbol_bits_num;
    let mut current_symbol = symbol;
    
    while remaining_bits > 0 {
        let bit = current_symbol & 1;
        current_symbol >>= 1;
        price += cmpt_price_one_bit(enc_ctx, symbol_probs[i as usize], bit);
        i = (i << 1) + bit;
        remaining_bits -= 1;
    }
    
    price
}
