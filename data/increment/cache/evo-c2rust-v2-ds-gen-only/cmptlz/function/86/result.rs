pub fn CmptLzGetMatchLenCoderProb(mut probsMatrix: Ptr<CmptLzDecProb>) -> Ptr<CmptLzDecProb> {
    return (probsMatrix + CMPTLZ_MATCH_LEN_CODER!()).cast();
}
