pub fn CmptlzDpInitMatch(mut encCtx: Ptr<CmptLzEncCtx>, mut matchesCount: u32, mut normalMatchPrice: u32, mut posState: u32, mut len: u32) {
    let mut i: u32 = 0;
    while (len > encCtx.matches[i].len).as_bool() {
        i += 1;
    }
    loop {
        let mut dist: u32 = encCtx.matches[i].dist.cast();
        let mut curAndLenPrice: u32 = (normalMatchPrice + CmptPriceDistWithLen(encCtx.cast(), dist.cast(), len.cast(), posState.cast())).cast();
        if (curAndLenPrice < encCtx.opts[len].price).as_bool() {
            encCtx.opts[len].price = curAndLenPrice.cast();
            encCtx.opts[len].posPrev = 0;
            encCtx.opts[len].backPrev = (dist + CMPTLZ_NUM_REPS!()).cast();
        }
        if (len == encCtx.matches[i].len).as_bool() {
            i += 1;
            if (i == matchesCount).as_bool() {
                break;
            }
        }
        len += 1;
    }
}
