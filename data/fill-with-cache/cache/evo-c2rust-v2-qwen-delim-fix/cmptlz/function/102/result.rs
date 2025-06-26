pub fn CmptPriceLongRep(mut encCtx: Ptr<CmptLzEncCtx>, mut longRepIndex: u32, mut state: CmptlzState, mut posState: u32) -> u32 {
    let mut price: u32 = 0;
    if longRepIndex == 0 {
        price = CmptPriceBit0(encCtx.cast(), encCtx.isRepG0[state].cast()).cast() + CmptPriceBit1(encCtx.cast(), encCtx.isRep0Long[state][posState].cast()).cast();
        break;
    } else if longRepIndex == 1 {
        price = CmptPriceBit1(encCtx.cast(), encCtx.isRepG0[state].cast()).cast() + CmptPriceBit0(encCtx.cast(), encCtx.isRepG1[state].cast()).cast();
        break;
    } else if longRepIndex == 2 {
        price = CmptPriceBit1(encCtx.cast(), encCtx.isRepG0[state].cast()).cast() + CmptPriceBit1(encCtx.cast(), encCtx.isRepG1[state].cast()).cast() + CmptPriceBit0(encCtx.cast(), encCtx.isRepG2[state].cast()).cast();
        break;
    } else if longRepIndex == 3 {
        price = CmptPriceBit1(encCtx.cast(), encCtx.isRepG0[state].cast()).cast() + CmptPriceBit1(encCtx.cast(), encCtx.isRepG1[state].cast()).cast() + CmptPriceBit1(encCtx.cast(), encCtx.isRepG2[state].cast()).cast();
        break;
    } else {
        break;
    }
    return price.cast();
}