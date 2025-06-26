pub fn CmptlzDp(mut encCtx: Ptr<CmptLzEncCtx>, mut mf: Ptr<CmptMfCtx>, mut position: u32) {
    let mut curIndex: u32 = encCtx.optsCurIndex;
    let mut endIndex: u32 = encCtx.optEndIndex;
    if endIndex != curIndex {
        encCtx.lenRes = encCtx.opts[curIndex].posPrev - curIndex;
        encCtx.backRes = encCtx.opts[curIndex].backPrev;
        encCtx.optsCurIndex = encCtx.opts[curIndex].posPrev;
        return;
    }
    let mut lenEnd: u32 = CmptlzDpInit(encCtx, mf, position);
    if lenEnd == CMPTLZ_UINT32_MAX!() {
        return;
    }
    let mut mainReps: Array<u32, { CMPTLZ_NUM_REPS!() }> = Default::default();
    c_memcpy_s!(mainReps, c_sizeofval!(mainReps), encCtx.reps, c_sizeofval!(encCtx.reps));
    let mut cur: u32;
    c_for!(cur = 1; cur < lenEnd; cur += 1; {
        encCtx.longestMatchLen = CmptlzMatchFinder(mf, c_ref!(encCtx.matchesCount), encCtx.matches.cast());
        if encCtx.longestMatchLen >= mf.niceLen {
            break;
        }
        CmptlzDpPre(encCtx, mainReps.cast(), cur);
        lenEnd = CmptlzDpProcess(encCtx, mf, mainReps.cast(), lenEnd, position + cur, cur);
    });
    CmptlzDpReverse(encCtx, cur);
    return;
}