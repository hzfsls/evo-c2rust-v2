pub fn CmptPriceLongRep(mut encCtx: Ptr<CmptLzEncCtx>, mut longRepIndex: u32, mut state: CmptlzState, mut posState: u32) -> u32 {
    let mut price: u32 = 0;
    c_switch!(longRepIndex;
        0 => {
            let tmp0 = state;
            price = CmptPriceBit0(encCtx, encCtx.isRepG0[state]) + CmptPriceBit1(encCtx, encCtx.isRep0Long[state][posState]);
            break;
        },
        1 => {
            price = (CmptPriceBit1(encCtx, encCtx.isRepG0[state]) + CmptPriceBit0(encCtx, encCtx.isRepG1[state]));
            break;
        },
        2 => {
            price = (CmptPriceBit1(encCtx, encCtx.isRepG0[state]) + CmptPriceBit1(encCtx, encCtx.isRepG1[state]) + CmptPriceBit0(encCtx, encCtx.isRepG2[state]));
            break;
        },
        3 => {
            price = (CmptPriceBit1(encCtx, encCtx.isRepG0[state]) + CmptPriceBit1(encCtx, encCtx.isRepG1[state]) + CmptPriceBit1(encCtx, encCtx.isRepG2[state]));
            break;
        },
        _ => {
            break;
        },
    );
    return price;
}
