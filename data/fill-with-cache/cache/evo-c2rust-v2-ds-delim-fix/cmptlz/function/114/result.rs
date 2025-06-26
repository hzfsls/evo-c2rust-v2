pub fn CmptlzEncLit(mut encCtx: Ptr<CmptLzEncCtx>, mut mf: Ptr<CmptMfCtx>, mut nowpos32: u32) -> i32 {
    let mut shiftRes: i32 = CMPT_OK!();
    let mut rc: Ptr<CmptRcCtx> = encCtx.rcCtx.cast();
    let mut posState: u32 = (nowpos32 & encCtx.pbMask).cast();
    let mut range: u32;
    let mut bit0Prob: u32;
    let mut newBound: u32;
    range = rc.range.cast();
    let mut probs: Ptr<CmptlzProb> = c_ref!(encCtx.isMatch[encCtx.state][posState]).cast();
    CMPT_RC_GET_NEWBOUND!(probs, bit0Prob, range, newBound);
    CMPT_RC_BIT_0_PROCESS!(rc, probs, newBound, range, bit0Prob, shiftRes);
    CMPTLZ_RETURN_IF_NOT_OK!(shiftRes);
    rc.range = range.cast();
    let mut litProb: Ptr<CmptlzProb> = c_ref!(encCtx.litMarcov.literal[0][0]).cast();
    let mut curByte: u8 = mf.srcStart[mf.readPos - mf.readAhead].cast();
    probs = CMPT_LIT_PROB_GET!(encCtx, litProb, nowpos32, mf.srcStart[mf.readPos - mf.readAhead - 1]);
    let mut state: CmptlzState = encCtx.state.cast();
    CMPT_STATE_UPDATE_WHEN_LIT!(encCtx.state);
    if (state < 7).as_bool() {
        shiftRes = CmptRcLitProcess(rc.cast(), probs.cast(), curByte.cast()).cast();
        CMPTLZ_RETURN_IF_NOT_OK!(shiftRes);
    } else {
        let mut match_byte: u8 = mf.srcStart[mf.readPos - encCtx.reps[0] - 1 - mf.readAhead].cast();
        shiftRes = CmptRcLitAfterMatch(rc.cast(), probs.cast(), curByte.cast(), match_byte.cast()).cast();
        CMPTLZ_RETURN_IF_NOT_OK!(shiftRes);
    }
    return CMPT_OK!();
}
