pub fn CmptLzGetIsMatchProb(mut probsMatrix: Ptr<CmptLzDecProb>) -> Ptr<CmptLzDecProb> {
    return (probsMatrix + CMPTLZ_IS_MATCH!()).cast();
}
