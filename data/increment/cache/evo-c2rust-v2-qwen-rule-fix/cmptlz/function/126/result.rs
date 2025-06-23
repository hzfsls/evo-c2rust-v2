pub fn CmptlzEncLongRep(mut encCtx: Ptr<CmptLzEncCtx>, mut repIndex: u32, mut nowpos32: u32, mut lenRes: u32) -> i32 {
    let mut shiftRes: i32 = CMPT_OK!();
    let mut posState: u32 = nowpos32 & encCtx.pbMask;
    let mut range: u32 = encCtx.rcCtx.range;
    let mut bit0Prob: u32 = Default::default();
    let mut newBound: u32 = Default::default();
    let mut realDist: u32 = Default::default();
    let mut probs: Ptr<CmptlzProb> = Default::default();
    probs = &encCtx.isMatch[encCtx.state][posState];
    CMPT_RC_GET_NEWBOUND!(probs, bit0Prob, range, newBound);
    CMPT_RC_BIT_1_PROCESS!(encCtx.rcCtx, probs, newBound, range, bit0Prob, shiftRes);
    if CMPTLZ_RETURN_IF_NOT_OK!(shiftRes).as_bool() {
        return shiftRes;
    }
    let tmp0 = encCtx.state;
    probs = &encCtx.isRep[tmp0];
    CMPT_RC_GET_NEWBOUND!(probs, bit0Prob, range, newBound);
    CMPT_RC_BIT_1_PROCESS!(encCtx.rcCtx, probs, newBound, range, bit0Prob, shiftRes);
    if CMPTLZ_RETURN_IF_NOT_OK!(shiftRes).as_bool() {
        return shiftRes;
    }
    probs = &encCtx.isRepG0[encCtx.state];
    CMPT_RC_GET_NEWBOUND!(probs, bit0Prob, range, newBound);
    match repIndex {
        0 => {
            CMPT_RC_BIT_0_PROCESS!(encCtx.rcCtx, probs, newBound, range, bit0Prob, shiftRes);
            if CMPTLZ_RETURN_IF_NOT_OK!(shiftRes).as_bool() {
                return shiftRes;
            }
            probs = &encCtx.isRep0Long[encCtx.state][posState];
            CMPT_RC_GET_NEWBOUND!(probs, bit0Prob, range, newBound);
            CMPT_RC_BIT_1!(encCtx.rcCtx, probs, newBound, range, bit0Prob);
        },
        1 => {
            CMPT_RC_BIT_1_PROCESS!(encCtx.rcCtx, probs, newBound, range, bit0Prob, shiftRes);
            if CMPTLZ_RETURN_IF_NOT_OK!(shiftRes).as_bool() {
                return shiftRes;
            }
            probs = &encCtx.isRepG1[encCtx.state];
            CMPT_RC_GET_NEWBOUND!(probs, bit0Prob, range, newBound);
            CMPT_RC_BIT_0!(probs, newBound, range, bit0Prob);
            realDist = encCtx.reps[1];
            encCtx.reps[1] = encCtx.reps[0];
            encCtx.reps[0] = realDist;
        },
        2 => {
            CMPT_RC_BIT_1_PROCESS!(encCtx.rcCtx, probs, newBound, range, bit0Prob, shiftRes);
            if CMPTLZ_RETURN_IF_NOT_OK!(shiftRes).as_bool() {
                return shiftRes;
            }
            probs = &encCtx.isRepG1[encCtx.state];
            CMPT_RC_GET_NEWBOUND!(probs, bit0Prob, range, newBound);
            CMPT_RC_BIT_1_PROCESS!(encCtx.rcCtx, probs, newBound, range, bit0Prob, shiftRes);
            if CMPTLZ_RETURN_IF_NOT_OK!(shiftRes).as_bool() {
                return shiftRes;
            }
            probs = &encCtx.isRepG2[encCtx.state];
            CMPT_RC_GET_NEWBOUND!(probs, bit0Prob, range, newBound);
            CMPT_RC_BIT_0!(probs, newBound, range, bit0Prob);
            realDist = encCtx.reps[2];
            encCtx.reps[2] = encCtx.reps[1];
            encCtx.reps[1] = encCtx.reps[0];
            encCtx.reps[0] = realDist;
        },
        3 => {
            CMPT_RC_BIT_1_PROCESS!(encCtx.rcCtx, probs, newBound, range, bit0Prob, shiftRes);
            if CMPTLZ_RETURN_IF_NOT_OK!(shiftRes).as_bool() {
                return shiftRes;
            }
            probs = &encCtx.isRepG1[encCtx.state];
            CMPT_RC_GET_NEWBOUND!(probs, bit0Prob, range, newBound);
            CMPT_RC_BIT_1_PROCESS!(encCtx.rcCtx, probs, newBound, range, bit0Prob, shiftRes);
            if CMPTLZ_RETURN_IF_NOT_OK!(shiftRes).as_bool() {
                return shiftRes;
            }
            probs = &encCtx.isRepG2[encCtx.state];
            CMPT_RC_GET_NEWBOUND!(probs, bit0Prob, range, newBound);
            CMPT_RC_BIT_1!(encCtx.rcCtx, probs, newBound, range, bit0Prob);
            realDist = encCtx.reps[3];
            encCtx.reps[3] = encCtx.reps[2];
            encCtx.reps[2] = encCtx.reps[1];
            encCtx.reps[1] = encCtx.reps[0];
            encCtx.reps[0] = realDist;
        },
        _ => {
            // default case
        },
    }
    CMPT_RC_NORMALIZE!(encCtx.rcCtx, range, shiftRes);
    if CMPTLZ_RETURN_IF_NOT_OK!(shiftRes).as_bool() {
        return shiftRes;
    }
    encCtx.rcCtx.range = range;
    shiftRes = CmptRcLenProcess(&encCtx.repLenEncoder, encCtx.rcCtx, lenRes, posState);
    if CMPTLZ_RETURN_IF_NOT_OK!(shiftRes).as_bool() {
        return shiftRes;
    }
    encCtx.repLenPriceCount -= 1;
    let mut state: CmptlzState = encCtx.state;
    encCtx.state = CMPT_STATE_UPDATE_WHEN_LONGREP!(state);
    return CMPT_OK!();
}