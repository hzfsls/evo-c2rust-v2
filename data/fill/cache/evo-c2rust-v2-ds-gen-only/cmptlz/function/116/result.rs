pub fn CmptlzEncNormalMatch(mut encCtx: Ptr<CmptLzEncCtx>, mut nowpos32: u32, mut backRes: u32, mut lenRes: u32) -> i32 {
    let mut shiftRes: i32 = CMPT_OK!();
    let mut posState: u32 = (nowpos32 & encCtx.pbMask).cast();
    let mut range: u32;
    let mut bit0Prob: u32;
    let mut newBound: u32;
    range = encCtx.rcCtx.range.cast();
    let mut probs: Ptr<CmptlzProb> = c_ref!(encCtx.isMatch[encCtx.state][posState]).cast();
    CMPT_RC_GET_NEWBOUND!(probs, bit0Prob, range, newBound);
    CMPT_RC_BIT_1_PROCESS!(encCtx.rcCtx, probs, newBound, range, bit0Prob, shiftRes);
    CMPTLZ_RETURN_IF_NOT_OK!(shiftRes);
    probs = c_ref!(encCtx.isRep[encCtx.state]).cast();
    CMPT_RC_GET_NEWBOUND!(probs, bit0Prob, range, newBound);
    CMPT_RC_BIT_0_PROCESS!(encCtx.rcCtx, probs, newBound, range, bit0Prob, shiftRes);
    CMPTLZ_RETURN_IF_NOT_OK!(shiftRes);
    encCtx.rcCtx.range = range.cast();
    let mut state: CmptlzState = encCtx.state.cast();
    encCtx.state = CMPT_STATE_UPDATE_WHEN_MATCH!(state).cast();
    shiftRes = CmptRcLenProcess(c_ref!(encCtx.matchLenEncoder).cast(), encCtx.rcCtx.cast(), lenRes.cast(), posState.cast()).cast();
    CMPTLZ_RETURN_IF_NOT_OK!(shiftRes);
    backRes -= CMPTLZ_NUM_REPS!();
    encCtx.reps[3] = encCtx.reps[2].cast();
    encCtx.reps[2] = encCtx.reps[1].cast();
    encCtx.reps[1] = encCtx.reps[0].cast();
    encCtx.reps[0] = backRes.cast();
    encCtx.matchPriceCount += 1;
    let mut posSlot: u32 = PosSloter(backRes.cast()).cast();
    shiftRes = CmptRcPosSlotProcess(encCtx.cast(), posSlot.cast(), lenRes.cast()).cast();
    CMPTLZ_RETURN_IF_NOT_OK!(shiftRes);
    if (backRes >= 4).as_bool() {
        shiftRes = CmptRcDistProcess(encCtx.cast(), posSlot.cast(), backRes.cast()).cast();
        CMPTLZ_RETURN_IF_NOT_OK!(shiftRes);
    }
    return CMPT_OK!();
}
