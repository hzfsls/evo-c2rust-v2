pub fn CmptLzGetPosSlotProb(mut probsMatrix: Ptr<CmptLzDecProb>) -> Ptr<CmptLzDecProb> {
    probsMatrix + CMPTLZ_POSSLOT!()
}