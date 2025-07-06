pub fn CmptRcReverseProcess(mut rcCtx: Ptr<CmptRcCtx>, mut probs: Ptr<CmptlzProb>, mut numBits: u32, mut sym: u32) -> i32 {
    let mut shiftRes: i32 = CMPT_OK!();
    let mut range: u32 = rcCtx.range;
    let mut bit0Prob: u32;
    let mut newBound: u32;
    let mut bit: u32;
    let mut m: u32 = 1;
    loop {
        bit = sym & 1;
        sym >>= 1;
        CMPT_RC_BIT_PROCESS!(rcCtx, probs + m, bit, bit0Prob, range, newBound, shiftRes);
        CMPTLZ_RETURN_IF_NOT_OK!(shiftRes);
        m = (m << 1) | bit;
        numBits -= 1;
        if numBits == 0 {
            break;
        }
    }
    rcCtx.range = range;
    return CMPT_OK!();
}