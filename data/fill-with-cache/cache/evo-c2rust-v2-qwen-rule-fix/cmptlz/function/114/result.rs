pub fn CmptlzEncLit(mut encCtx: Ptr<CmptLzEncCtx>, mut mf: Ptr<CmptMfCtx>, mut nowpos32: u32) -> i32 {
    let mut shiftRes: i32 = CMPT_OK!();
    let mut rc: Ptr<CmptRcCtx> = encCtx.rcCtx;
    let mut posState: u32 = nowpos32 & encCtx.pbMask;
    let mut range: u32 = rc.range;
    let mut bit0Prob: CmptlzProb = Default::default();
    let mut newBound: u32 = Default::default();
    let tmp0 = encCtx.state;
    let mut probs: Ptr<CmptlzProb> = &encCtx.isMatch[tmp0][posState];
    CMPT_RC_GET_NEWBOUND!(probs, bit0Prob, range, newBound);
    CMPT_RC_BIT_0_PROCESS!(rc, probs, newBound, range, bit0Prob, shiftRes);
    CMPTLZ_RETURN_IF_NOT_OK!(shiftRes);
    rc.range = range;
    let mut litProb: Ptr<CmptlzProb> = &encCtx.litMarcov.literal[0][0];
    let mut curByte: u8 = mf.srcStart[mf.readPos - mf.readAhead];
    probs = CMPT_LIT_PROB_GET!(encCtx, litProb, nowpos32, mf.srcStart[mf.readPos - mf.readAhead - 1].cast());
    let mut state: CmptlzState = encCtx.state;
    CMPT_STATE_UPDATE_WHEN_LIT!(encCtx.state);
    if (state < 7) {
        shiftRes = CmptRcLitProcess(rc, probs, curByte.cast());
        CMPTLZ_RETURN_IF_NOT_OK!(shiftRes);
    } else {
        let mut match_byte: u8 = mf.srcStart[mf.readPos - encCtx.reps[0] - 1 - mf.readAhead];
        shiftRes = CmptRcLitAfterMatch(rc, probs, curByte.cast(), match_byte.cast());
        CMPTLZ_RETURN_IF_NOT_OK!(shiftRes);
    }
    return CMPT_OK!();
}