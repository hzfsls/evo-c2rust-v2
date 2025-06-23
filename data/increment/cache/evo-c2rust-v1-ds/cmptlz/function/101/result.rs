pub fn CmptLzDecGetProbsInit(mut decCtx: Ptr<CmptLzDecCtx>) {
    let mut idx: u32;
    let mut numProbs: u32 = CmptLzGetNumProbs(c_ref!(decCtx.prop).cast()).cast();
    let mut decProbs: Ptr<CmptLzDecProb> = decCtx.probs.cast();

    c_for!(idx = 0; idx < numProbs; idx.suffix_plus_plus(); {
        decProbs[idx] = (CMPTLZ_PROB_LG!() >> 1).cast();
    });
    decCtx.state = 0;
}
