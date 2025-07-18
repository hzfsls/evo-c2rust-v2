pub fn CmptRcLitProcess(mut rcCtx: Ptr<CmptRcCtx>, mut prob: Ptr<CmptlzProb>, mut sym: u32) -> i32 {
    let mut shiftRes: i32 = CMPT_OK!();
    let mut range: u32 = rcCtx.range;
    let mut bit0Prob: u32 = Default::default();
    let mut newBound: u32 = Default::default();
    let mut curBit: u32 = Default::default();

    sym |= 0x100;
    c_for!(; sym < 0x10000; sym <<= 1; {
        let mut litProbTableIndex: Ptr<CmptlzProb> = prob + (sym >> 8);
        curBit = (sym >> 7) & 1;
        CMPT_RC_BIT_PROCESS!(rcCtx, litProbTableIndex, curBit, bit0Prob, range, newBound, shiftRes);
        CMPTLZ_RETURN_IF_NOT_OK!(shiftRes);
    });
    rcCtx.range = range;
    return CMPT_OK!();
}