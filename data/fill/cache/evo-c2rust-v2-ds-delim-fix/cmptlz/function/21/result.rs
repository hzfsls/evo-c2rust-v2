pub fn CmptLzGetPosSlotProb(mut probsMatrix: Ptr<CmptLzDecProb>) -> Ptr<CmptLzDecProb> {
    return (probsMatrix + CMPTLZ_POSSLOT!()).cast();
}
