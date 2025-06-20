pub fn CmptLzDecGetProbsInit(mut decCtx: Ptr<CmptLzDecCtx>) {
    let mut idx: u32;
    let mut numProbs: u32 = CmptLzGetNumProbs(c_ref!(decCtx.prop));
    let mut decProbs: Ptr<CmptLzDecProb> = decCtx.probs;
    c_for!(idx = 0; idx < numProbs; idx += 1; {
        decProbs[idx] = CMPTLZ_PROB_LG!() >> 1;
    });
    decCtx.state = 0;
}