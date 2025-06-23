pub fn CmptLzGetLiteralProb(mut probsMatrix: Ptr<CmptLzDecProb>) -> Ptr<CmptLzDecProb> {
    return (probsMatrix + CMPTLZ_LITERAL!()).cast();
}
