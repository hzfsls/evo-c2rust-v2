pub fn CmptlzDpInitLongRep(mut encCtx: Ptr<CmptLzEncCtx>, mut repLens: Ptr<u32>, mut repMatchPrice: u32, mut posState: u32) {
    let mut i: u32;
    c_for!(i = 0; i < CMPTLZ_NUM_REPS!(); i += 1; {
        let mut repLen: u32 = repLens[i];
        if repLen < CMPTLZ_MATCH_LEN_MIN!() {
            continue;
        }
        let mut price: u32 = repMatchPrice + CmptPriceLongRep(encCtx, i, encCtx.state, posState);
        loop {
            let mut curAndLenPrice: u32 = price + CmptPriceLen(c_ref!(encCtx.repLenEncoder), repLen, posState);
            if curAndLenPrice < encCtx.opts[repLen].price {
                encCtx.opts[repLen].price = curAndLenPrice;
                encCtx.opts[repLen].posPrev = 0;
                encCtx.opts[repLen].backPrev = i;
            }
            repLen -= 1;
            if repLen < CMPTLZ_MATCH_LEN_MIN!() {
                break;
            }
        }
    });
}