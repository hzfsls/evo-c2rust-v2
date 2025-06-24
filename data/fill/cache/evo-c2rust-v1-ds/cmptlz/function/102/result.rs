pub fn CmptPriceLongRep(mut encCtx: Ptr<CmptLzEncCtx>, mut longRepIndex: u32, mut state: CmptlzState, mut posState: u32) -> u32 {
    let mut price: u32 = 0;
    c_switch!(longRepIndex; 0 => {
        price = CmptPriceBit0(encCtx.cast(), encCtx.isRepG0[state].cast()).cast() + CmptPriceBit1(encCtx.cast(), encCtx.isRep0Long[state][posState].cast()).cast();
        break;
    }, 1 => {
        price = CmptPriceBit1(encCtx.cast(), encCtx.isRepG0[state].cast()).cast() + CmptPriceBit0(encCtx.cast(), encCtx.isRepG1[state].cast()).cast();
        break;
    }, 2 => {
        price = CmptPriceBit1(encCtx.cast(), encCtx.isRepG0[state].cast()).cast() + CmptPriceBit1(encCtx.cast(), encCtx.isRepG1[state].cast()).cast() + CmptPriceBit0(encCtx.cast(), encCtx.isRepG2[state].cast()).cast();
        break;
    }, 3 => {
        price = CmptPriceBit1(encCtx.cast(), encCtx.isRepG0[state].cast()).cast() + CmptPriceBit1(encCtx.cast(), encCtx.isRepG1[state].cast()).cast() + CmptPriceBit1(encCtx.cast(), encCtx.isRepG2[state].cast()).cast();
        break;
    }, _ => {
        break;
    }, );
    return price.cast();
}
