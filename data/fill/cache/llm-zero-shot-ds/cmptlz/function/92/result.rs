fn cmpt_price_symbol(enc_ctx: &mut CmptLzEncCtx, symbol_probs: &[CmptlzProb], symbol_bits_num: u32, symbol: u32) -> u32 {
    let mut price = 0;
    let mut symbol = symbol + (1 << symbol_bits_num);
    while symbol != 1 {
        let bit = symbol & 1;
        symbol >>= 1;
        price += cmpt_price_one_bit(enc_ctx, symbol_probs[symbol as usize], bit);
    }
    price
}
