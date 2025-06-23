pub fn CmptlzDpInitLongRep(mut encCtx: Ptr<CmptLzEncCtx>, mut repLens: Ptr<u32>, mut repMatchPrice: u32, mut posState: u32) {
    let mut i: u32;
    c_for!(i = 0; i < CMPTLZ_NUM_REPS!(); i.suffix_plus_plus(); {
        let mut repLen: u32 = repLens[i];
        if (repLen < CMPTLZ_MATCH_LEN_MIN!()).as_bool() {
            continue;
        }
        let mut price: u32 = repMatchPrice + CmptPriceLongRep(encCtx.cast(), i.cast(), encCtx.state.cast(), posState.cast());
        c_do!({
            let mut curAndLenPrice: u32 = price + CmptPriceLen(c_ref!(encCtx.repLenEncoder).cast(), repLen.cast(), posState.cast());
            if (curAndLenPrice < encCtx.opts[repLen].price).as_bool() {
                encCtx.opts[repLen].price = curAndLenPrice.cast();
                encCtx.opts[repLen].posPrev = 0;
                encCtx.opts[repLen].backPrev = i.cast();
            }
            repLen -= 1;
        } while repLen >= CMPTLZ_MATCH_LEN_MIN!());
    });
}
