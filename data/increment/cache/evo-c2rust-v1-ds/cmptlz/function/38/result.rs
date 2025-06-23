pub fn CmptlzEncodeIO(mut encCtx: Ptr<CmptLzEncCtx>, mut dest: Ptr<u8>, mut destLen: Ptr<usize>, mut src: Ptr<u8>, mut srcLen: usize, mut alloc: Ptr<CmptLzMemHook>) -> i32 {
    let mut res: i32;

    res = CmptMfPrepare(encCtx.cast(), src.cast(), srcLen.cast(), alloc.cast()).cast();
    if res != 0 {
        CMPTLZ_LOG!(res, cstr!("CmptMfPrepare Fail!"));
        CmptlzFreeAll(encCtx.cast(), alloc.cast());
        return res;
    }

    res = CmptRcPrepare(encCtx.cast(), dest.cast(), destLen.cast(), alloc.cast()).cast();
    if res != 0 {
        CMPTLZ_LOG!(res, cstr!("CmptRcPrepare Fail!"));
        CmptlzFreeAll(encCtx.cast(), alloc.cast());
        return res;
    }

    CmptlzEncPrepare(encCtx.cast());

    res = CmptEncodeAll(encCtx.cast()).cast();

    if res != 0 {
        CmptlzFreeAll(encCtx.cast(), alloc.cast());
        CMPTLZ_LOG!(res, cstr!("CmptEncode Process Fail!"));
        return res;
    }

    *destLen -= encCtx.rcCtx.outBufLeft.cast();

    if encCtx.nowpos64 != srcLen {
        CMPTLZ_LOG!(res, cstr!("CmptEncode FileSize Fail!"));
        CmptlzFreeAll(encCtx.cast(), alloc.cast());
        return CMPT_ENC_ERROR_FILESIZE!();
    }

    CmptlzFreeAll(encCtx.cast(), alloc.cast());
    return res.cast();
}
