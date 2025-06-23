pub fn CmptLzGetRepLenCoderProb(mut probsMatrix: Ptr<CmptLzDecProb>) -> Ptr<CmptLzDecProb> {
    return (probsMatrix + CMPTLZ_REP_LEN_CODER!()).cast();
}
