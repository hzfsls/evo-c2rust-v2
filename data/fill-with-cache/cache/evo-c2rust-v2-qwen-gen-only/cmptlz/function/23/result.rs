pub fn CmptLzGetAilgnProb(mut probsMatrix: Ptr<CmptLzDecProb>) -> Ptr<CmptLzDecProb> {
    return probsMatrix + CMPTLZ_ALIGN!();
}