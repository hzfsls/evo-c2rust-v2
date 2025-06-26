pub fn CmptlzEncodeIO(mut encCtx: Ptr<CmptLzEncCtx>, mut dest: Ptr<u8>, mut destLen: Ptr<usize>, mut src: Ptr<u8>, mut srcLen: usize, mut alloc: Ptr<CmptLzMemHook>) -> i32 {
    let mut res: i32;
    res = CmptMfPrepare(encCtx, src, srcLen, alloc);
    if (res != 0) {
        CMPTLZ_LOG!(res, cstr!("CmptMfPrepare Fail!"));
        CmptlzFreeAll(encCtx, alloc);
        return res;
    }
    res = CmptRcPrepare(encCtx, dest, destLen, alloc);
    if (res != 0) {
        CMPTLZ_LOG!(res, cstr!("CmptRcPrepare Fail!"));
        CmptlzFreeAll(encCtx, alloc);
        return res;
    }
    CmptlzEncPrepare(encCtx);
    res = CmptEncodeAll(encCtx);
    if (res != 0) {
        CmptlzFreeAll(encCtx, alloc);
        CMPTLZ_LOG!(res, cstr!("CmptEncode Process Fail!"));
        return res;
    }
    *destLen = (*destLen).cast::<usize>() - encCtx.rcCtx.outBufLeft.cast::<usize>();
    if (encCtx.nowpos64 != srcLen) {
        CMPTLZ_LOG!(res, cstr!("CmptEncode FileSize Fail!"));
        CmptlzFreeAll(encCtx, alloc);
        return CMPT_ENC_ERROR_FILESIZE!();
    }
    CmptlzFreeAll(encCtx, alloc);
    return res;
}