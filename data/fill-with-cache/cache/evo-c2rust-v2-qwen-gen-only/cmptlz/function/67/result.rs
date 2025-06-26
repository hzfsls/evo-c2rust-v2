pub fn CmptlzDp(mut encCtx: Ptr<CmptLzEncCtx>, mut mf: Ptr<CmptMfCtx>, mut position: u32) {
    let mut curIndex: u32 = encCtx.optsCurIndex.cast();
    let mut endIndex: u32 = encCtx.optEndIndex.cast();
    if (endIndex != curIndex).as_bool() {
        encCtx.lenRes = encCtx.opts[curIndex].posPrev - curIndex;
        encCtx.backRes = encCtx.opts[curIndex].backPrev.cast();
        encCtx.optsCurIndex = encCtx.opts[curIndex].posPrev.cast();
        return;
    }
    let mut lenEnd: u32 = CmptlzDpInit(encCtx.cast(), mf.cast(), position.cast()).cast();
    if (lenEnd == CMPTLZ_UINT32_MAX!()).as_bool() {
        return;
    }
    let mut mainReps: Array<u32, { CMPTLZ_NUM_REPS!() }> = Default::default();
    c_memcpy_s!(mainReps.cast(), c_sizeofval!(mainReps).cast(), encCtx.reps.cast(), c_sizeofval!(encCtx.reps).cast()).cast::<Void>();
    let mut cur: u32 = 1;
    c_for!(; cur < lenEnd; cur.suffix_plus_plus(); {
        encCtx.longestMatchLen = CmptlzMatchFinder(mf.cast(), c_ref!(encCtx.matchesCount).cast(), encCtx.matches.cast()).cast();
        if (encCtx.longestMatchLen >= mf.niceLen).as_bool() {
            break;
        }
        CmptlzDpPre(encCtx.cast(), mainReps.cast(), cur.cast());
        lenEnd = CmptlzDpProcess(encCtx.cast(), mf.cast(), mainReps.cast(), lenEnd.cast(), position + cur.cast(), cur.cast()).cast();
    });
    CmptlzDpReverse(encCtx.cast(), cur.cast());
    return;
}