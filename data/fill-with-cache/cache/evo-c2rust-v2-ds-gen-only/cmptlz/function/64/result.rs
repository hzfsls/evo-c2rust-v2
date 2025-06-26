pub fn CmptlzDpTryCurAndMatch(mut encCtx: Ptr<CmptLzEncCtx>, mut startLen: u32, mut matchCount: u32, mut normalmatch_prefixPrice: u32, mut cur: u32, mut posState: u32) {
    let mut i: u32 = 0;
    while (startLen > encCtx.matches[i].len).as_bool() {
        i += 1;
    }
    let mut lenTest: u32;
    c_for!(lenTest = startLen; ; lenTest += 1; {
        let mut curBack: u32 = encCtx.matches[i].dist.cast();
        let mut cur_normalmatchPrice: u32 = (normalmatch_prefixPrice + CmptPriceDistWithLen(encCtx.cast(), curBack.cast(), lenTest.cast(), posState.cast())).cast();
        if (cur_normalmatchPrice < encCtx.opts[cur + lenTest].price).as_bool() {
            encCtx.opts[cur + lenTest].price = cur_normalmatchPrice.cast();
            encCtx.opts[cur + lenTest].posPrev = cur.cast();
            encCtx.opts[cur + lenTest].backPrev = (curBack + CMPTLZ_NUM_REPS!()).cast();
        }
        if (lenTest == encCtx.matches[i].len).as_bool() {
            if (i.prefix_plus_plus() == matchCount).as_bool() {
                break;
            }
        }
    });
}
