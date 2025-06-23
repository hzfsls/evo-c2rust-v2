fn cmpt_price_set(enc_ctx: &mut CmptLzEncCtx, probs: &[CmptlzProb], start_price: u32, prices: &mut [u32]) {
    for i in (0..8).step_by(2) {
        let mut price = start_price;
        price += cmpt_price_one_bit(enc_ctx, probs[1], (i >> 2));
        price += cmpt_price_one_bit(enc_ctx, probs[2 + (i >> 2)], (i >> 1) & 1);
        let prob = probs[4 + (i >> 1)];
        prices[i] = price + cmpt_price_bit0(enc_ctx, prob);
        prices[i + 1] = price + cmpt_price_bit1(enc_ctx, prob);
    }
}
