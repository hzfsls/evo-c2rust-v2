pub fn CmptLzDecFreeProbs(mut decCtx: Ptr<CmptLzDecCtx>, mut memHook: Ptr<CmptLzMemHook>) {
    if (decCtx.probs != NULL!()) {
        CmptLzDecMemFree(memHook, CMPTLZ_PROB_HANDLE!(), decCtx.probs.cast::<Ptr<Void>>());
        decCtx.probs = NULL!();
    }
}
