pub fn CmptPriceLongRep(mut encCtx: Ptr<CmptLzEncCtx>, mut longRepIndex: u32, mut state: CmptlzState, mut posState: u32) -> u32 {
    let mut price: u32 = 0;
    if longRepIndex == 0 {
        price = CmptPriceBit0(encCtx, encCtx.isRepG0[state]) + CmptPriceBit1(encCtx, encCtx.isRep0Long[state][posState]);
        break;
    } else if longRepIndex == 1 {
        let tmp0 = state;
        price = CmptPriceBit1(encCtx, encCtx.isRepG0[tmp0]);
        break;
    } else if longRepIndex == 2 {
        price = CmptPriceBit1(encCtx, encCtx.isRepG0[state]) + CmptPriceBit1(encCtx, encCtx.isRepG1[state]) + CmptPriceBit0(encCtx, encCtx.isRepG2[state]);
        break;
    } else if longRepIndex == 3 {
        price = CmptPriceBit1(encCtx, encCtx.isRepG0[state]) + CmptPriceBit1(encCtx, encCtx.isRepG1[state]) + CmptPriceBit1(encCtx, encCtx.isRepG2[state]);
        break;
    } else {
        break;
    }
    return price;
}