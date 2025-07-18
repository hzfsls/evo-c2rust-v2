pub fn CmptlzDpInitMatch(mut encCtx: Ptr<CmptLzEncCtx>, mut matchesCount: u32, mut normalMatchPrice: u32, mut posState: u32, mut len: u32) {
    let mut i: u32 = 0;
    while len > encCtx.matches[i].len {
        i += 1;
    }
    c_for!(;; len += 1; {
        let dist: u32 = encCtx.matches[i].dist;
        let curAndLenPrice: u32 = normalMatchPrice + CmptPriceDistWithLen(encCtx, dist, len, posState);
        if curAndLenPrice < encCtx.opts[len].price {
            encCtx.opts[len].price = curAndLenPrice;
            encCtx.opts[len].posPrev = 0;
            encCtx.opts[len].backPrev = dist + CMPTLZ_NUM_REPS!();
        }
        if len == encCtx.matches[i].len {
            i += 1;
            if i == matchesCount {
                break;
            }
        }
    });
}