pub fn CmptLzGetIsRepG0Prob(mut probsMatrix: Ptr<CmptLzDecProb>) -> Ptr<CmptLzDecProb> {
    return (probsMatrix + CMPTLZ_ISREPG0!()).cast();
}
