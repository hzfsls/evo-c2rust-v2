pub fn CmptLzDecFree(mut decCtx: Ptr<CmptLzDecCtx>, mut memHook: Ptr<CmptLzMemHook>) -> i32 {
    if decCtx == NULL!() || memHook == NULL!() {
        return CMPT_ERROR_UNSUPPORTED!();
    }

    CmptLzDecFreeProbs(decCtx.cast(), memHook.cast());
    CmptLzFreeDict(decCtx.cast(), memHook.cast());

    return CMPT_OK!();
}
