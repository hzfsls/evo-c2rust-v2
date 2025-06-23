pub fn CmptlzDpProcess(mut encCtx: Ptr<CmptLzEncCtx>, mut mf: Ptr<CmptMfCtx>, mut mainReps: Ptr<u32>, mut lenEnd: u32, mut position: u32, mut cur: u32) -> u32 {
    let mut curState: CmptlzState = encCtx.opts[cur].state.cast();
    let mut bufAvailFull: u32 = CMPTLZ_FIND_MIN!(CmptMfAvail(mf).cast() + 1, CMPT_DP_OPTMAX!() - 1 - cur).cast();
    let mut buf: Ptr<u8> = (CmptMfGetPtr(mf).cast::<Ptr<u8>>() - 1).cast();
    let mut niceLen: u32 = mf.niceLen.cast();
    let mut curPrice: u32 = encCtx.opts[cur].price.cast();
    let mut curByte: u8 = (*buf).cast();
    let mut latestMatchByte: u8 = (*(buf - mainReps[0] - 1)).cast();
    let mut posState: u32 = position & encCtx.posMask.cast();
    encCtx.litMarcov.pos = position.cast();
    encCtx.litMarcov.prevByte = (*(buf - 1)).cast();
    CmptlzDpTryCurAndLit(encCtx.cast(), curPrice.cast(), curState.cast(), posState.cast(), cur.cast(), latestMatchByte.cast(), curByte.cast());
    let mut matchPrice: u32 = curPrice + CmptPriceBit1(encCtx.cast(), encCtx.isMatch[curState][posState].cast()).cast();
    let mut repMatchPrice: u32 = matchPrice + CmptPriceBit1(encCtx.cast(), encCtx.isRep[curState].cast()).cast();
    if (curByte == latestMatchByte).as_bool() && (!(encCtx.opts[cur + 1].posPrev < cur).as_bool() && encCtx.opts[cur + 1].backPrev == 0).as_bool() {
        CmptlzDpTryCurAndShort(encCtx.cast(), repMatchPrice.cast(), cur.cast(), curState.cast(), posState.cast());
    }
    if (bufAvailFull < CMPTLZ_MATCH_LEN_MIN!()).as_bool() {
        return lenEnd.cast();
    }
    let mut bufAvail: u32 = CMPTLZ_FIND_MIN!(bufAvailFull.cast(), niceLen.cast()).cast();
    let mut startLen: u32 = CMPTLZ_MATCH_LEN_MIN!().cast();
    let mut mainRepIndex: u32;
    c_for!(mainRepIndex = 0; mainRepIndex < CMPTLZ_NUM_REPS!(); mainRepIndex.suffix_plus_plus(); {
        let mut bufRepBack: Ptr<u8> = (buf - mainReps[mainRepIndex] - 1).cast();
        if NOT_EQUAL_2_BYTES!(buf.cast(), bufRepBack.cast()).as_bool() {
            continue;
        }
        let mut lenEqual: u32;
        lenEqual = CmptMemCmpLenSafe(buf.cast(), bufRepBack.cast(), CMPTLZ_MATCH_LEN_MIN!().cast(), bufAvail.cast()).cast();
        while (lenEnd < cur + lenEqual).as_bool() {
            lenEnd.suffix_plus_plus();
            encCtx.opts[lenEnd].price = CMPT_INFINITY_PRICE!().cast();
        }
        let mut lenEqualMem: u32 = lenEqual.cast();
        let mut prefixPrice: u32 = repMatchPrice + CmptPriceLongRep(encCtx.cast(), mainRepIndex.cast(), curState.cast(), posState.cast()).cast();
        CmptlzDpTryCurAndLong(encCtx.cast(), prefixPrice.cast(), cur.cast(), mainRepIndex.cast(), lenEqual.cast(), posState.cast());
        lenEqual = lenEqualMem.cast();
        if (mainRepIndex == 0).as_bool() {
            startLen = lenEqual + 1;
        }
    });
    let mut newLongestLen: u32 = encCtx.longestMatchLen.cast();
    let mut matchCount: u32 = encCtx.matchesCount.cast();
    if (newLongestLen > bufAvail).as_bool() {
        newLongestLen = bufAvail.cast();
        matchCount = 0;
        while (newLongestLen > encCtx.matches[matchCount].len).as_bool() {
            matchCount.suffix_plus_plus();
        }
        encCtx.matches[matchCount.suffix_plus_plus()].len = newLongestLen.cast();
    }
    if (newLongestLen >= startLen).as_bool() {
        let mut normalmatch_prefixPrice: u32 = matchPrice + CmptPriceBit0(encCtx.cast(), encCtx.isRep[curState].cast()).cast();
        while (lenEnd < cur + newLongestLen).as_bool() {
            lenEnd.suffix_plus_plus();
            encCtx.opts[lenEnd].price = CMPT_INFINITY_PRICE!().cast();
        }
        CmptlzDpTryCurAndMatch(encCtx.cast(), startLen.cast(), matchCount.cast(), normalmatch_prefixPrice.cast(), cur.cast(), posState.cast());
    }
    return lenEnd.cast();
}