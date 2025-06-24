pub fn CmptPriceShortRep(mut encCtx: Ptr<CmptLzEncCtx>, mut state: CmptlzState, mut posState: u32) -> u32 {
    return (CmptPriceBit0(encCtx.cast(), encCtx.isRepG0[state].cast()).cast::<u32>() + CmptPriceBit0(encCtx.cast(), encCtx.isRep0Long[state][posState].cast()).cast::<u32>()).cast();
}
