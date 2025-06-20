pub fn CmptPriceShortRep(mut encCtx: Ptr<CmptLzEncCtx>, mut state: CmptlzState, mut posState: u32) -> u32 {
    return CmptPriceBit0(encCtx, encCtx.isRepG0[state]) + CmptPriceBit0(encCtx, encCtx.isRep0Long[state][posState]);
}