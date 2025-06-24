pub fn CmptLzDecAllocateProbs(mut decCtx: Ptr<CmptLzDecCtx>, mut decProt: Ptr<CmptLzDecProt>, mut memHook: Ptr<CmptLzMemHook>) -> i32 {
    let mut numProbs: u32 = CmptLzGetNumProbs(decProt.cast()).cast();
    if decCtx.probs == NULL!() {
        decCtx.probs = CmptLzDecMemAlloc(memHook.cast(), CMPTLZ_PROB_HANDLE!(), numProbs * c_sizeof!(CmptLzDecProb)).cast::<Ptr<CmptLzDecProb>>();
    } else {
        if numProbs != decCtx.numProbs {
            CmptLzDecFreeProbs(decCtx.cast(), memHook.cast());
            decCtx.probs = CmptLzDecMemAlloc(memHook.cast(), CMPTLZ_PROB_HANDLE!(), numProbs * c_sizeof!(CmptLzDecProb)).cast::<Ptr<CmptLzDecProb>>();
        }
    }
    if decCtx.probs == NULL!() {
        return CMPT_ERROR_MEM!();
    }
    decCtx.probsPlus1664 = (decCtx.probs + 1664).cast();
    decCtx.numProbs = numProbs.cast();
    return CMPT_OK!();
}
