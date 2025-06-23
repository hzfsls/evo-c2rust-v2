pub fn CmptLzGetIsRepG2Prob(mut probsMatrix: Ptr<CmptLzDecProb>) -> Ptr<CmptLzDecProb> {
    return (probsMatrix + CMPTLZ_ISREPG2!()).cast();
}
