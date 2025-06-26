fn cmptlz_flush(enc_ctx: &mut CmptLzEncCtx) -> i32 {
    enc_ctx.enc_need_finish = true;
    if enc_ctx.end_marker != 0 {
        cmptlz_end_marker();
    }
    cmpt_rc_flush_data(enc_ctx.rc_ctx);
    cmpt_rc_flush_64kb(enc_ctx.rc_ctx)
}
