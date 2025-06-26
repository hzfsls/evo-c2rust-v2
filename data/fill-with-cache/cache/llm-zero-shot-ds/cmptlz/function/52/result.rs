static mut CMPT_PRICE_COUNT: u32 = /* appropriate value */;

fn cmpt_price_check(enc_ctx: &mut CmptLzEncCtx) {
    if enc_ctx.match_price_count >= CMPT_PRICE_COUNT {
        cmpt_price_gen_dist_table(enc_ctx);
        cmpt_price_gen_align_table(enc_ctx);
        cmpt_price_gen_len_table(enc_ctx, &mut enc_ctx.match_len_encoder);
    }
    if enc_ctx.rep_len_price_count <= 0 {
        enc_ctx.rep_len_price_count = CMPT_PRICE_COUNT;
        cmpt_price_gen_len_table(enc_ctx, &mut enc_ctx.rep_len_encoder);
    }
}
