pub fn cmptlz_free_all(enc_ctx: Option<&mut CmptLzEncCtx>, alloc: &mut CmptLzMemHook) {
    let Some(enc_ctx) = enc_ctx else {
        return;
    };

    if let Some(mf_ctx) = enc_ctx.mf_ctx.take() {
        if let Some(hash) = mf_ctx.hash.take() {
            alloc.cmpt_lz_free(CMPTLZ_MF_HASH_HANDLE, hash);
        }
        if let Some(son) = mf_ctx.son.take() {
            alloc.cmpt_lz_free(CMPTLZ_MF_SON_HANDLE, son);
        }
        alloc.cmpt_lz_free(CMPTLZ_MF_CCTX_HANDLE, mf_ctx);
    }

    if let Some(rc_ctx) = enc_ctx.rc_ctx.take() {
        if let Some(buf_base) = rc_ctx.buf_base.take() {
            alloc.cmpt_lz_free(CMPTLZ_RC_BUF_HANDLE, buf_base);
        }
        alloc.cmpt_lz_free(CMPTLZ_RC_CCTX_HANDLE, rc_ctx);
    }

    alloc.cmpt_lz_free(CMPTLZ_ENC_CCTX_HANDLE, enc_ctx);
}
