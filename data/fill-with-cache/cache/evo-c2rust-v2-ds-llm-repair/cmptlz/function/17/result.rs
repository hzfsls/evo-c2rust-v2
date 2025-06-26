pub fn CmptLzGetIsRepG1Prob(mut probsMatrix: Ptr<CmptLzDecProb>) -> Ptr<CmptLzDecProb> {
    return (probsMatrix + CMPTLZ_ISREPG1!()).cast();
}
