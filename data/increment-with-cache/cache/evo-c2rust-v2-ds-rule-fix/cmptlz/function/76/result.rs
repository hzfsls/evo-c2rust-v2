pub fn CmptLzGetIsRepProb(mut probsMatrix: Ptr<CmptLzDecProb>) -> Ptr<CmptLzDecProb> {
    return (probsMatrix + CMPTLZ_ISREP!()).cast();
}
