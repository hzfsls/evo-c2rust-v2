pub fn CmptLzGetIsRepG0LongProb(mut probsMatrix: Ptr<CmptLzDecProb>) -> Ptr<CmptLzDecProb> {
    return (probsMatrix + CMPTLZ_REP0_LONG!()).cast();
}
