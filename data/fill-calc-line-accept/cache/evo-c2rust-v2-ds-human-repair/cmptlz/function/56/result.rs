pub fn CmptlzDpInitShortRep(mut encCtx: Ptr<CmptLzEncCtx>, mut repMatchPrice: u32, mut posState: u32) {
    let mut shortRepPrice: u32 = (repMatchPrice + CmptPriceShortRep(encCtx.cast(), encCtx.state.cast(), posState.cast())).cast();
    if (shortRepPrice < encCtx.opts[1].price).as_bool() {
        encCtx.opts[1].price = shortRepPrice.cast();
        encCtx.opts[1].backPrev = 0;
    }
}
