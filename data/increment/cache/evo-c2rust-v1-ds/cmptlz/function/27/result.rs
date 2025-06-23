pub fn CmptlzDp(mut encCtx: Ptr<CmptLzEncCtx>, mut mf: Ptr<CmptMfCtx>, mut position: u32) {
    let mut curIndex: u32 = encCtx.optsCurIndex.cast();
    let mut endIndex: u32 = encCtx.optEndIndex.cast();

    if endIndex != curIndex {
        encCtx.lenRes = (encCtx.opts[curIndex].posPrev - curIndex).cast();
        encCtx.backRes = encCtx.opts[curIndex].backPrev.cast();
        encCtx.optsCurIndex = encCtx.opts[curIndex].posPrev.cast();
        return;
    }
    let mut lenEnd: u32 = CmptlzDpInit(encCtx.cast(), mf.cast(), position.cast()).cast();

    if lenEnd == CMPTLZ_UINT32_MAX!() {
        return;
    }
    let mut mainReps: Array<u32, { CMPTLZ_NUM_REPS!() }> = Default::default();
    c_memcpy_s!(mainReps, c_sizeofval!(mainReps), encCtx.reps, c_sizeofval!(encCtx.reps));

    let mut cur: u32 = 1;
    while cur < lenEnd {
        encCtx.longestMatchLen = CmptlzMatchFinder(mf.cast(), c_ref!(encCtx.matchesCount).cast(), encCtx.matches.cast()).cast();
        if encCtx.longestMatchLen >= mf.niceLen {
            break;
        }
        CmptlzDpPre(encCtx.cast(), mainReps.cast(), cur.cast());
        lenEnd = CmptlzDpProcess(encCtx.cast(), mf.cast(), mainReps.cast(), lenEnd.cast(), (position + cur).cast(), cur.cast()).cast();
        cur += 1;
    }
    CmptlzDpReverse(encCtx.cast(), cur.cast());
    return;
}
