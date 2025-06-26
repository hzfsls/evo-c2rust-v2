pub fn CmptLzGetNumProbs(mut decProt: Ptr<CmptLzDecProt>) -> u32 {
    return (NUM_BASE_PROBS!() + (0x300 << (decProt.litCtx + decProt.litPos))).cast();
}
