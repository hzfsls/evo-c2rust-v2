pub fn CmptLzDecFreeProbs(mut decCtx: Ptr<CmptLzDecCtx>, mut memHook: Ptr<CmptLzMemHook>) {
    if (decCtx.probs != NULL!()).as_bool() {
        CmptLzDecMemFree(memHook.cast(), CMPTLZ_PROB_HANDLE!(), decCtx.probs.cast());
        decCtx.probs = NULL!();
    }
}
