pub fn CmptLzDecGetProbsInit(mut decCtx: Ptr<CmptLzDecCtx>) {
    let mut idx: u32 = Default::default();
    let mut numProbs: u32 = CmptLzGetNumProbs(c_ref!(decCtx.prop));
    let mut decProbs: Ptr<CmptLzDecProb> = decCtx.probs;
    c_for!(idx = 0; idx < numProbs; idx.suffix_plus_plus(); {
        decProbs[idx] = (CMPTLZ_PROB_LG!() >> 1);
    });
    decCtx.state = 0;
}