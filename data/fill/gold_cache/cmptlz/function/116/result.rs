pub fn CmptlzEncNormalMatch(mut encCtx: Ptr<CmptLzEncCtx>, mut nowpos32: u32, mut backRes: u32, mut lenRes: u32) -> i32 {
    let mut shiftRes: i32 = CMPT_OK!();
    let mut posState: u32 = nowpos32 & encCtx.pbMask as u32;
    let mut range: u32;
    let mut bit0Prob: u32;
    let mut newBound: u32;
    range = encCtx.rcCtx.range;
    let idx = encCtx.state;
    let mut probs: Ptr<CmptlzProb> = c_ref!(encCtx.isMatch[idx][posState]);
    CMPT_RC_GET_NEWBOUND!(probs, bit0Prob, range, newBound);
    CMPT_RC_BIT_1_PROCESS!(encCtx.rcCtx, probs, newBound, range, bit0Prob, shiftRes);
    CMPTLZ_RETURN_IF_NOT_OK!(shiftRes);
    let idx = encCtx.state;
    probs = c_ref!(encCtx.isRep[idx]);
    CMPT_RC_GET_NEWBOUND!(probs, bit0Prob, range, newBound);
    CMPT_RC_BIT_0_PROCESS!(encCtx.rcCtx, probs, newBound, range, bit0Prob, shiftRes);
    CMPTLZ_RETURN_IF_NOT_OK!(shiftRes);
    encCtx.rcCtx.range = range;
    let mut state: CmptlzState = encCtx.state;
    encCtx.state = CMPT_STATE_UPDATE_WHEN_MATCH!(state);
    shiftRes = CmptRcLenProcess(c_ref!(encCtx.matchLenEncoder), encCtx.rcCtx, lenRes, posState as u64);
    CMPTLZ_RETURN_IF_NOT_OK!(shiftRes);
    backRes -= CMPTLZ_NUM_REPS!();
    encCtx.reps[3] = encCtx.reps[2];
    encCtx.reps[2] = encCtx.reps[1];
    encCtx.reps[1] = encCtx.reps[0];
    encCtx.reps[0] = backRes;
    encCtx.matchPriceCount += 1;
    let mut posSlot: u32 = PosSloter(backRes);
    shiftRes = CmptRcPosSlotProcess(encCtx, posSlot, lenRes);
    CMPTLZ_RETURN_IF_NOT_OK!(shiftRes);
    if backRes >= 4 {
        shiftRes = CmptRcDistProcess(encCtx, posSlot, backRes);
        CMPTLZ_RETURN_IF_NOT_OK!(shiftRes);
    }
    return CMPT_OK!();
}