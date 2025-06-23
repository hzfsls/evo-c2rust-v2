pub fn CmptLzDistDecHelper(mut decCtx: Ptr<CmptLzDecCtx>, mut distDec: u32, mut bufToDec: Ptr<u8>, mut pRange: Ptr<u32>, mut pRangeCode: Ptr<u32>, mut pRangeBound: Ptr<u32>, mut range: u32, mut rangeCode: u32, mut rangeBound: u32) {
    decCtx.reps[CMPTLZ_REP3!()] = decCtx.reps[CMPTLZ_REP2!()].cast();
    decCtx.reps[CMPTLZ_REP2!()] = decCtx.reps[1].cast();
    decCtx.reps[1] = decCtx.reps[0].cast();
    decCtx.reps[0] = (distDec + 1).cast();

    decCtx.buf = bufToDec.cast();
    decCtx.state = if decCtx.state < CMPTLZ_LIT_STATES!() { CMPTLZ_LIT_STATES!() } else { CMPTLZ_LIT_STATES!() + CMPTLZ_REP3!() };
    *pRange = range.cast();
    *pRangeCode = rangeCode.cast();
    *pRangeBound = rangeBound.cast();
}
