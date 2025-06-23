pub fn CmptLzGetSpecPosProb(mut probsMatrix: Ptr<CmptLzDecProb>) -> Ptr<CmptLzDecProb> {
    return (probsMatrix + CMPTLZ_SPEC_POS!()).cast();
}
