pub fn cmptlz_price_init(enc_ctx: &mut CmptLzEncCtx) {
    cmpt_price_gen_root_table(enc_ctx);
    cmpt_price_gen_dist_table(enc_ctx);
    cmpt_price_gen_align_table(enc_ctx);
}
