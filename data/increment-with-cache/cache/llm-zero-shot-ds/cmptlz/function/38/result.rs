pub fn cmptlz_encode_io(
    enc_ctx: &mut CmptLzEncCtx,
    dest: &mut [u8],
    dest_len: &mut usize,
    src: &[u8],
    src_len: usize,
    alloc: &mut CmptLzMemHook,
) -> i32 {
    let mut res;

    res = cmpt_mf_prepare(enc_ctx, src, src_len, alloc);
    if res != 0 {
        cmptlz_log(res, "CmptMfPrepare Fail!");
        cmptlz_free_all(enc_ctx, alloc);
        return res;
    }

    res = cmpt_rc_prepare(enc_ctx, dest, dest_len, alloc);
    if res != 0 {
        cmptlz_log(res, "CmptRcPrepare Fail!");
        cmptlz_free_all(enc_ctx, alloc);
        return res;
    }

    cmptlz_enc_prepare(enc_ctx);

    res = cmpt_encode_all(enc_ctx);

    if res != 0 {
        cmptlz_free_all(enc_ctx, alloc);
        cmptlz_log(res, "CmptEncode Process Fail!");
        return res;
    }

    *dest_len -= enc_ctx.rc_ctx.out_buf_left;

    if enc_ctx.nowpos64 != src_len {
        cmptlz_log(res, "CmptEncode FileSize Fail!");
        cmptlz_free_all(enc_ctx, alloc);
        return CMPT_ENC_ERROR_FILESIZE;
    }

    cmptlz_free_all(enc_ctx, alloc);
    res
}
