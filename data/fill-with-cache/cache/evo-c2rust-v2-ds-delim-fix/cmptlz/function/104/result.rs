pub fn CmptRcLitProcess(mut rcCtx: Ptr<CmptRcCtx>, mut prob: Ptr<CmptlzProb>, mut sym: u32) -> i32 {
    let mut shiftRes: i32 = CMPT_OK!();
    let mut range: u32 = rcCtx.range;
    let mut bit0Prob: u32;
    let mut newBound: u32;
    let mut curBit: u32;
    sym |= 0x100;
    while (sym < 0x10000).as_bool() {
        let mut litProbTableIndex: Ptr<CmptlzProb> = (prob + (sym >> 8)).cast();
        curBit = (sym >> 7) & 1;
        CMPT_RC_BIT_PROCESS!(rcCtx, litProbTableIndex, curBit, bit0Prob, range, newBound, shiftRes);
        CMPTLZ_RETURN_IF_NOT_OK!(shiftRes);
        sym <<= 1;
    }
    rcCtx.range = range.cast();
    return CMPT_OK!();
}
