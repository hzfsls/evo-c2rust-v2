pub fn cmpt_price_gen_align_table(enc_ctx: &mut CmptLzEncCtx) {
    for i in 0..(1 << CMPTLZ_ALIGN_BITS) {
        enc_ctx.price_align_table[i as usize] = cmpt_price_symbol_reverse(
            enc_ctx,
            &enc_ctx.prob_align,
            CMPTLZ_ALIGN_BITS,
            i,
        );
    }
}
