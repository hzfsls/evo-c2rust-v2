pub fn CmptLzDistDecHelper(mut decCtx: Ptr<CmptLzDecCtx>, mut distDec: u32, mut bufToDec: Ptr<u8>, mut pRange: Ptr<u32>,
                           mut pRangeCode: Ptr<u32>, mut pRangeBound: Ptr<u32>, mut range: u32, mut rangeCode: u32,
                           mut rangeBound: u32) {
    decCtx.reps[CMPTLZ_REP3!()] = decCtx.reps[CMPTLZ_REP2!()];
    decCtx.reps[CMPTLZ_REP2!()] = decCtx.reps[1];
    decCtx.reps[1] = decCtx.reps[0];
    decCtx.reps[0] = distDec + 1;
    decCtx.buf = bufToDec;
    decCtx.state = if decCtx.state < CMPTLZ_LIT_STATES!() { CMPTLZ_LIT_STATES!() } else { CMPTLZ_LIT_STATES!() + CMPTLZ_REP3!() };
    *pRange = range;
    *pRangeCode = rangeCode;
    *pRangeBound = rangeBound;
}