pub fn CmptlzEncShortRep(mut encCtx: Ptr<CmptLzEncCtx>, mut nowpos32: u32) -> i32 {
    let mut shiftRes: i32 = CMPT_OK!();
    let mut posState: u32 = nowpos32 & encCtx.pbMask;
    let mut range: u32;
    let mut bit0Prob: u32;
    let mut newBound: u32;
    range = encCtx.rcCtx.range;
    let mut probs: Ptr<CmptlzProb> = c_ref!(encCtx.isMatch[encCtx.state][posState]).cast();
    CMPT_RC_GET_NEWBOUND!(probs, bit0Prob, range, newBound);
    CMPT_RC_BIT_1_PROCESS!(encCtx.rcCtx, probs, newBound, range, bit0Prob, shiftRes);
    CMPTLZ_RETURN_IF_NOT_OK!(shiftRes);

    probs = c_ref!(encCtx.isRep[encCtx.state]).cast();
    CMPT_RC_GET_NEWBOUND!(probs, bit0Prob, range, newBound);
    CMPT_RC_BIT_1_PROCESS!(encCtx.rcCtx, probs, newBound, range, bit0Prob, shiftRes);
    CMPTLZ_RETURN_IF_NOT_OK!(shiftRes);

    probs = c_ref!(encCtx.isRepG0[encCtx.state]).cast();
    CMPT_RC_GET_NEWBOUND!(probs, bit0Prob, range, newBound);
    CMPT_RC_BIT_0_PROCESS!(encCtx.rcCtx, probs, newBound, range, bit0Prob, shiftRes);
    CMPTLZ_RETURN_IF_NOT_OK!(shiftRes);

    probs = c_ref!(encCtx.isRep0Long[encCtx.state][posState]).cast();
    CMPT_RC_GET_NEWBOUND!(probs, bit0Prob, range, newBound);
    CMPT_RC_BIT_0_PROCESS!(encCtx.rcCtx, probs, newBound, range, bit0Prob, shiftRes);
    CMPTLZ_RETURN_IF_NOT_OK!(shiftRes);
    encCtx.rcCtx.range = range;

    let mut state: CmptlzState = encCtx.state;
    encCtx.state = CMPT_STATE_UPDATE_WHEN_SHORTREP!(state);
    return CMPT_OK!();
}
