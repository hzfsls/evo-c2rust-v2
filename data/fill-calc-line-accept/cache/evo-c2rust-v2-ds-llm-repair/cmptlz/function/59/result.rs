pub fn CmptlzDpInit(mut encCtx: Ptr<CmptLzEncCtx>, mut mf: Ptr<CmptMfCtx>, mut position: u32) -> u32 {
    let niceLen: u32 = mf.niceLen;
    let mut lenMain: u32;
    let mut matchesCount: u32 = 0;
    if (mf.readAhead == 0) {
        lenMain = CmptlzMatchFinder(mf, c_ref!(matchesCount), encCtx.matches.cast());
    } else {
        lenMain = encCtx.longestMatchLen;
        matchesCount = encCtx.matchesCount;
    }
    let buf: Ptr<u8> = CmptMfGetPtr(mf) - 1;
    let bufAvail: u32 = CMPTLZ_FIND_MIN!(CmptMfAvail(mf) + 1, CMPT_MF_LONGEST_MATCH!());
    if (bufAvail < CMPTLZ_MATCH_LEN_MIN!()) {
        encCtx.backRes = CMPTLZ_UINT32_MAX!();
        encCtx.lenRes = 1;
        return CMPTLZ_UINT32_MAX!();
    }
    let mut repLens: Array<u32, { CMPTLZ_NUM_REPS!() }> = Default::default();
    let mut repMaxIndex: u32 = 0;
    let mut i: u32;
    c_for!(i = 0; i < CMPTLZ_NUM_REPS!(); i.suffix_plus_plus(); {
        let bufBack: Ptr<u8> = buf - encCtx.reps[i] - 1;
        if NOT_EQUAL_2_BYTES!(buf, bufBack) {
            repLens[i] = 0;
            continue;
        }
        repLens[i] = CmptMemCmpLenSafe(buf, bufBack, CMPTLZ_MATCH_LEN_MIN!(), bufAvail);
        if (repLens[i] > repLens[repMaxIndex]) {
            repMaxIndex = i;
        }
    });
    if (repLens[repMaxIndex] >= niceLen) {
        encCtx.backRes = repMaxIndex;
        encCtx.lenRes = repLens[repMaxIndex];
        CmptlzMatchSkiper(mf, (repLens[repMaxIndex] - 1));
        return CMPTLZ_UINT32_MAX!();
    }
    if (lenMain >= niceLen) {
        encCtx.backRes = (encCtx.matches[matchesCount - 1].dist + CMPTLZ_NUM_REPS!());
        encCtx.lenRes = lenMain;
        CmptlzMatchSkiper(mf, (lenMain - 1));
        return CMPTLZ_UINT32_MAX!();
    }
    let currentByte: u8 = *buf;
    let matchByte: u8 = *(buf - encCtx.reps[0] - 1);
    let lenEnd: u32 = CMPTLZ_FIND_MAX!(lenMain, repLens[repMaxIndex]);
    if (lenEnd < CMPTLZ_MATCH_LEN_MIN!()) && (currentByte != matchByte) {
        encCtx.backRes = CMPTLZ_UINT32_MAX!();
        encCtx.lenRes = 1;
        return CMPTLZ_UINT32_MAX!();
    }
    encCtx.opts[0].state = encCtx.state;
    let posState: u32 = position & encCtx.posMask;
    encCtx.litMarcov.pos = position;
    encCtx.litMarcov.prevByte = *(buf - 1);
    let isLiteralState: bool = (encCtx.state < 7);
    let isMatchMode: bool = !isLiteralState;
    encCtx.opts[1].price = (CmptPriceBit0(encCtx, encCtx.isMatch[encCtx.state][posState]) +
                            CmptPriceLiteral(encCtx, isMatchMode, matchByte.cast(), currentByte.cast()));
    encCtx.opts[1].backPrev = CMPTLZ_UINT32_MAX!();
    let tmp0 = encCtx.state;
    let matchPrice: u32 = CmptPriceBit1(encCtx, encCtx.isMatch[tmp0][posState]);
    let repMatchPrice: u32 = (matchPrice + CmptPriceBit1(encCtx, encCtx.isRep[encCtx.state]));
    if (matchByte == currentByte) {
        CmptlzDpInitShortRep(encCtx, repMatchPrice, posState);
    }
    if (lenEnd < CMPTLZ_MATCH_LEN_MIN!()) {
        encCtx.backRes = encCtx.opts[1].backPrev;
        encCtx.lenRes = 1;
        return CMPTLZ_UINT32_MAX!();
    }
    encCtx.opts[1].posPrev = 0;
    c_for!(i = 0; i < CMPTLZ_NUM_REPS!(); i.suffix_plus_plus(); {
        encCtx.opts[0].backs[i] = encCtx.reps[i];
    });
    let mut len: u32 = lenEnd;
    c_do!({
        encCtx.opts[len].price = CMPT_INFINITY_PRICE!();
        len.suffix_minus_minus();
    } while len >= CMPTLZ_MATCH_LEN_MIN!());
    CmptlzDpInitLongRep(encCtx, repLens.cast(), repMatchPrice, posState);
    let normalMatchPrice: u32 = (matchPrice + CmptPriceBit0(encCtx, encCtx.isRep[encCtx.state]));
    len = if repLens[0] > CMPTLZ_MATCH_LEN_MIN!() { repLens[0] + 1 } else { CMPTLZ_MATCH_LEN_MIN!() };
    if (len <= lenMain) {
        CmptlzDpInitMatch(encCtx, matchesCount, normalMatchPrice, posState, len);
    }
    return lenEnd;
}
