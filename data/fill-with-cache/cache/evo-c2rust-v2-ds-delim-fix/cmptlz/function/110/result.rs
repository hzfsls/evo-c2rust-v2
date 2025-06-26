pub fn CmptRcLenProcess(mut lenEncoder: Ptr<CmptLenEncoder>, mut rcCtx: Ptr<CmptRcCtx>, mut len: u32, mut posState: u64) -> i32 {
    let mut shiftRes: i32 = CMPT_OK!();
    let mut range: u32 = rcCtx.range.cast();
    let mut newBound: u32;
    let mut bit0Prob: u32;
    len -= CMPTLZ_MATCH_LEN_MIN!();
    let mut probs: Ptr<CmptlzProb> = lenEncoder.low.cast();
    CMPT_RC_GET_NEWBOUND!(probs, bit0Prob, range, newBound);
    if (len >= CMPT_LEN_BOUND!()).as_bool() {
        CMPT_RC_BIT_1_PROCESS!(rcCtx, probs, newBound, range, bit0Prob, shiftRes);
        if (shiftRes != CMPT_OK!()).as_bool() {
            return shiftRes;
        }
        probs += CMPT_LEN_BOUND!();
        CMPT_RC_GET_NEWBOUND!(probs, bit0Prob, range, newBound);
        if (len >= CMPT_LEN_BOUND!() * CMPT_DOUBLE!()).as_bool() {
            CMPT_RC_BIT_1_PROCESS!(rcCtx, probs, newBound, range, bit0Prob, shiftRes);
            if (shiftRes != CMPT_OK!()).as_bool() {
                return shiftRes;
            }
            rcCtx.range = range.cast();
            shiftRes = CmptRcLitProcess(rcCtx.cast(), lenEncoder.high.cast(), len - CMPT_LEN_BOUND!() * CMPT_DOUBLE!()).cast();
            if (shiftRes != CMPT_OK!()).as_bool() {
                return shiftRes;
            }
            return CMPT_OK!();
        }
        len -= CMPT_LEN_BOUND!();
    }
    let mut m: u32;
    let mut bit: u32;
    CMPT_RC_BIT_0_PROCESS!(rcCtx, probs, newBound, range, bit0Prob, shiftRes);
    if (shiftRes != CMPT_OK!()).as_bool() {
        return shiftRes;
    }
    probs += (posState << (1 + 3)).cast::<usize>();
    bit = (len >> 2).cast();
    CMPT_RC_BIT_PROCESS!(rcCtx, probs + 1, bit, bit0Prob, range, newBound, shiftRes);
    if (shiftRes != CMPT_OK!()).as_bool() {
        return shiftRes;
    }
    m = (1 << 1) + bit;
    bit = (len >> 1) & 1;
    CMPT_RC_BIT_PROCESS!(rcCtx, probs + m, bit, bit0Prob, range, newBound, shiftRes);
    if (shiftRes != CMPT_OK!()).as_bool() {
        return shiftRes;
    }
    m = (m << 1) + bit;
    bit = len & 1;
    CMPT_RC_BIT_PROCESS!(rcCtx, probs + m, bit, bit0Prob, range, newBound, shiftRes);
    if (shiftRes != CMPT_OK!()).as_bool() {
        return shiftRes;
    }
    rcCtx.range = range.cast();
    return CMPT_OK!();
}
