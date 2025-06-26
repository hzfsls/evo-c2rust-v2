pub fn CmptlzDpTryCurAndMatch(mut encCtx: Ptr<CmptLzEncCtx>, mut startLen: u32, mut matchCount: u32, mut normalmatch_prefixPrice: u32, mut cur: u32, mut posState: u32) {
    let mut i: u32 = 0;
    while (startLen > encCtx.matches[i].len) {
        i += 1;
    }
    let mut lenTest: u32;
    c_for!(lenTest = startLen; ; lenTest += 1; {
        let mut curBack: u32 = encCtx.matches[i].dist;
        let mut cur_normalmatchPrice: u32 = normalmatch_prefixPrice + CmptPriceDistWithLen(encCtx, curBack, lenTest, posState);
        if (cur_normalmatchPrice < encCtx.opts[cur + lenTest].price) {
            encCtx.opts[cur + lenTest].price = cur_normalmatchPrice;
            encCtx.opts[cur + lenTest].posPrev = cur;
            encCtx.opts[cur + lenTest].backPrev = (curBack + CMPTLZ_NUM_REPS!());
        }
        if (lenTest == encCtx.matches[i].len) {
            if (i.suffix_plus_plus() == matchCount) {
                break;
            }
        }
    });
}