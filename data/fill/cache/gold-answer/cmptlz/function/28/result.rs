pub fn CmptLzGetNumProbs(mut decProt: Ptr<CmptLzDecProt>) -> u32 {
    return NUM_BASE_PROBS!() as u32  + ((0x300 as u32) << (decProt.litCtx + decProt.litPos));
}