pub fn CmptLzGetProbsMatrix(mut decCtx: Ptr<CmptLzDecCtx>) -> Ptr<CmptLzDecProb> {
    return decCtx.probsPlus1664.cast();
}
