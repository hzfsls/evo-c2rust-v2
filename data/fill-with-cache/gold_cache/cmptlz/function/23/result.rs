pub fn CmptLzGetAilgnProb(mut probsMatrix: Ptr<CmptLzDecProb>) -> Ptr<CmptLzDecProb> {
    probsMatrix + CMPTLZ_ALIGN!()
}