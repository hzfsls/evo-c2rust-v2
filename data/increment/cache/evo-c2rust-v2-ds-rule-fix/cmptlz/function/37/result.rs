pub fn CmptlzFreeAll(mut encCtx: Ptr<CmptLzEncCtx>, mut alloc: Ptr<CmptLzMemHook>) {
    if (encCtx == NULL!()) {
        return;
    }
    if (encCtx.mfCtx != NULL!()) {
        if (encCtx.mfCtx.hash != NULL!()) {
            alloc.CmptLzFree(CMPTLZ_MF_HASH_HANDLE!(), encCtx.mfCtx.hash);
            encCtx.mfCtx.hash = NULL!();
        }
        if (encCtx.mfCtx.son != NULL!()) {
            alloc.CmptLzFree(CMPTLZ_MF_SON_HANDLE!(), encCtx.mfCtx.son);
            encCtx.mfCtx.son = NULL!();
        }
        alloc.CmptLzFree(CMPTLZ_MF_CCTX_HANDLE!(), encCtx.mfCtx);
        encCtx.mfCtx = NULL!();
    }
    if (encCtx.rcCtx != NULL!()) {
        if (encCtx.rcCtx.bufBase != NULL!()) {
            alloc.CmptLzFree(CMPTLZ_RC_BUF_HANDLE!(), encCtx.rcCtx.bufBase);
            encCtx.rcCtx.bufBase = NULL!();
        }
        alloc.CmptLzFree(CMPTLZ_RC_CCTX_HANDLE!(), encCtx.rcCtx);
        encCtx.rcCtx = NULL!();
    }
    alloc.CmptLzFree(CMPTLZ_ENC_CCTX_HANDLE!(), encCtx);
    encCtx = NULL!();
}
