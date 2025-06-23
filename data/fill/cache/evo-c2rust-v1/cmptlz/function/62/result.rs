pub fn CmptlzDpTryCurAndShort(mut encCtx: Ptr<CmptLzEncCtx>, mut repMatchPrice: u32, mut cur: u32, mut curState: CmptlzState, mut posState: u32) {
    let mut shortRepPrice: u32 = repMatchPrice + CmptPriceShortRep(encCtx.cast(), curState.cast(), posState.cast()).cast();
    if shortRepPrice < encCtx.opts[cur + 1].price {
        encCtx.opts[cur + 1].price = shortRepPrice.cast();
        encCtx.opts[cur + 1].posPrev = cur.cast();
        encCtx.opts[cur + 1].backPrev = 0;
    }
}
