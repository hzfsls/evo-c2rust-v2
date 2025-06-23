pub fn CmptRcReverseProcess(mut rcCtx: Ptr<CmptRcCtx>, mut probs: Ptr<CmptlzProb>, mut numBits: u32, mut sym: u32) -> i32 {
    let mut shiftRes: i32 = CMPT_OK!();
    let mut range: u32 = rcCtx.range.cast();
    let mut bit0Prob: u32 = Default::default();
    let mut newBound: u32 = Default::default();
    let mut bit: u32 = Default::default();
    let mut m: u32 = 1;
    c_do!({
        bit = sym & 1;
        sym >>= 1;
        CMPT_RC_BIT_PROCESS!(rcCtx, probs + m, bit, bit0Prob, range, newBound, shiftRes);
        CMPTLZ_RETURN_IF_NOT_OK!(shiftRes);
        m = (m << 1) | bit;
    } while numBits.prefix_minus_minus() != 0);

    rcCtx.range = range.cast();
    return CMPT_OK!();
}
