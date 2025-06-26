pub fn CmptlzDpInitShortRep(mut encCtx: Ptr<CmptLzEncCtx>, mut repMatchPrice: u32, mut posState: u32) {
    let mut shortRepPrice: u32 = repMatchPrice + CmptPriceShortRep(encCtx, encCtx.state, posState);
    if (shortRepPrice < encCtx.opts[1].price) {
        encCtx.opts[1].price = shortRepPrice;
        encCtx.opts[1].backPrev = 0;
    }
}