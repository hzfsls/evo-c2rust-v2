pub fn CmptLzDecAllocateProbs(mut decCtx: Ptr<CmptLzDecCtx>, mut decProt: Ptr<CmptLzDecProt>, mut memHook: Ptr<CmptLzMemHook>) -> i32 {
    let mut numProbs: u32 = CmptLzGetNumProbs(decProt);

    if (decCtx.probs == NULL!()) {
        decCtx.probs = CmptLzDecMemAlloc(memHook, CMPTLZ_PROB_HANDLE!(), numProbs * c_sizeof!(CmptLzDecProb)).cast::<Ptr<CmptLzDecProb>>();
    } else {
        if (numProbs != decCtx.numProbs) {
            CmptLzDecFreeProbs(decCtx, memHook);
            decCtx.probs = CmptLzDecMemAlloc(memHook, CMPTLZ_PROB_HANDLE!(), numProbs * c_sizeof!(CmptLzDecProb)).cast::<Ptr<CmptLzDecProb>>();
        }
    }

    if (decCtx.probs == NULL!()) {
        return CMPT_ERROR_MEM!();
    }

    decCtx.probsPlus1664 = decCtx.probs + 1664;
    decCtx.numProbs = numProbs;

    return CMPT_OK!();
}
