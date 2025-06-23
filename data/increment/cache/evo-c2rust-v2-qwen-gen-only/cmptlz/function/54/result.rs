pub fn CmptlzFreeAll(mut encCtx: Ptr<CmptLzEncCtx>, mut alloc: Ptr<CmptLzMemHook>) {
    if (encCtx == NULL!()).as_bool() {
        return;
    }

    if (encCtx.mfCtx != NULL!()).as_bool() {
        if (encCtx.mfCtx.hash != NULL!()).as_bool() {
            alloc.CmptLzFree(CMPTLZ_MF_HASH_HANDLE.cast(), encCtx.mfCtx.hash.cast());
            encCtx.mfCtx.hash = NULL!();
        }
        if (encCtx.mfCtx.son != NULL!()).as_bool() {
            alloc.CmptLzFree(CMPTLZ_MF_SON_HANDLE.cast(), encCtx.mfCtx.son.cast());
            encCtx.mfCtx.son = NULL!();
        }
        alloc.CmptLzFree(CMPTLZ_MF_CCTX_HANDLE.cast(), encCtx.mfCtx.cast());
        encCtx.mfCtx = NULL!();
    }

    if (encCtx.rcCtx != NULL!()).as_bool() {
        if (encCtx.rcCtx.bufBase != NULL!()).as_bool() {
            alloc.CmptLzFree(CMPTLZ_RC_BUF_HANDLE.cast(), encCtx.rcCtx.bufBase.cast());
            encCtx.rcCtx.bufBase = NULL!();
        }
        alloc.CmptLzFree(CMPTLZ_RC_CCTX_HANDLE.cast(), encCtx.rcCtx.cast());
        encCtx.rcCtx = NULL!();
    }

    alloc.CmptLzFree(CMPTLZ_ENC_CCTX_HANDLE.cast(), encCtx.cast());
    encCtx = NULL!();
}