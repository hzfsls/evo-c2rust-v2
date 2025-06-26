pub fn CmptRcLitAfterMatch(mut rcCtx: Ptr<CmptRcCtx>, mut prob: Ptr<CmptlzProb>, mut sym: u32, mut matchByte: u32) -> i32 {
    let mut shiftRes: i32 = CMPT_OK!();
    let mut range: u32 = rcCtx.range;
    let mut offs: u32 = 0x100;
    let mut bit0Prob: u32;
    let mut newBound: u32;
    let mut curBit: u32;
    c_for!(sym |= 0x100; sym < 0x10000; {
        matchByte <<= 1;
        let mut litProbTableIndex: Ptr<CmptlzProb> = (prob + (offs + (matchByte & offs) + (sym >> 8)).cast();
        curBit = (sym >> 7) & 1;
        sym <<= 1;
        offs &= !(matchByte ^ sym);
        CMPT_RC_BIT_PROCESS!(rcCtx, litProbTableIndex, curBit, bit0Prob, range, newBound, shiftRes);
        CMPTLZ_RETURN_IF_NOT_OK!(shiftRes);
    });
    rcCtx.range = range;
    return CMPT_OK!();
}
