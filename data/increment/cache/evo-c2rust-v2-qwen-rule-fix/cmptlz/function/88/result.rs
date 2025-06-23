pub fn CmptlzDpProcess(mut encCtx: Ptr<CmptLzEncCtx>, mut mf: Ptr<CmptMfCtx>, mut mainReps: Ptr<u32>, mut lenEnd: u32, mut position: u32, mut cur: u32) -> u32 {
    let mut curState: CmptlzState = encCtx.opts[cur].state;
    let mut bufAvailFull: u32 = CMPTLZ_FIND_MIN!(CmptMfAvail(mf) + 1, CMPT_DP_OPTMAX!() - 1 - cur);
    let mut buf: Ptr<u8> = (CmptMfGetPtr(mf).cast::<Ptr<u8>>() - 1);
    let mut niceLen: u32 = mf.niceLen;
    let mut curPrice: u32 = encCtx.opts[cur].price;
    let mut curByte: u8 = (*buf);
    let mut latestMatchByte: u8 = (*(buf - mainReps[0] - 1));
    let mut posState: u32 = position & encCtx.posMask;

    encCtx.litMarcov.pos = position;
    encCtx.litMarcov.prevByte = (*(buf - 1));

    CmptlzDpTryCurAndLit(encCtx, curPrice, curState, posState, cur, latestMatchByte, curByte);

    let mut matchPrice: u32 = (curPrice + CmptPriceBit1(encCtx, encCtx.isMatch[curState][posState]));
    let mut repMatchPrice: u32 = (matchPrice + CmptPriceBit1(encCtx, encCtx.isRep[curState]));

    let tmp0 = cur + 1;
    if (curByte == latestMatchByte) && (!(encCtx.opts[tmp0].backPrev == 0)) {
        CmptlzDpTryCurAndShort(encCtx, repMatchPrice, cur, curState, posState);
    }

    if (bufAvailFull < CMPTLZ_MATCH_LEN_MIN!()) {
        return lenEnd;
    }
    let mut bufAvail: u32 = CMPTLZ_FIND_MIN!(bufAvailFull, niceLen);
    let mut startLen: u32 = CMPTLZ_MATCH_LEN_MIN!();

    let mut mainRepIndex: u32;
    c_for!(mainRepIndex = 0; mainRepIndex < CMPTLZ_NUM_REPS!(); mainRepIndex.suffix_plus_plus(); {
        let mut bufRepBack: Ptr<u8> = (buf - mainReps[mainRepIndex] - 1);
        if NOT_EQUAL_2_BYTES!(buf, bufRepBack) {
            continue;
        }
        let mut lenEqual: u32 = CmptMemCmpLenSafe(buf, bufRepBack, CMPTLZ_MATCH_LEN_MIN!(), bufAvail);
        while (lenEnd < cur + lenEqual) {
            lenEnd.suffix_plus_plus();
            encCtx.opts[lenEnd].price = CMPT_INFINITY_PRICE!();
        }
        let mut lenEqualMem: u32 = lenEqual;
        let mut prefixPrice: u32 = (repMatchPrice + CmptPriceLongRep(encCtx, mainRepIndex, curState, posState));
        CmptlzDpTryCurAndLong(encCtx, prefixPrice, cur, mainRepIndex, lenEqual, posState);
        lenEqual = lenEqualMem;
        if (mainRepIndex == 0) {
            startLen = (lenEqual + 1);
        }
    });

    let mut newLongestLen: u32 = encCtx.longestMatchLen;
    let mut matchCount: u32 = encCtx.matchesCount;

    if (newLongestLen > bufAvail) {
        newLongestLen = bufAvail;
        matchCount = 0;
        while (newLongestLen > encCtx.matches[matchCount].len) {
            matchCount.suffix_plus_plus();
        }
        encCtx.matches[matchCount.suffix_plus_plus()].len = newLongestLen;
    }

    if (newLongestLen >= startLen) {
        let mut normalmatch_prefixPrice: u32 = (matchPrice + CmptPriceBit0(encCtx, encCtx.isRep[curState]));
        while (lenEnd < cur + newLongestLen) {
            lenEnd.suffix_plus_plus();
            encCtx.opts[lenEnd].price = CMPT_INFINITY_PRICE!();
        }
        CmptlzDpTryCurAndMatch(encCtx, startLen, matchCount, normalmatch_prefixPrice, cur, posState);
    }
    return lenEnd;
}