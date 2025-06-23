pub fn CmptLzRepDec(mut decCtx: Ptr<CmptLzDecCtx>, mut pRange: Ptr<u32>, mut pRangeCode: Ptr<u32>, mut pRangeBound: Ptr<u32>, mut dicPosLimit: usize, mut posState: u32) -> u32 {
    let mut repLen: u32;
    let mut repDist: u32;
    let mut mkState: u32 = decCtx.state.cast();
    let mut bufToDec: Ptr<u8> = decCtx.buf.cast();
    let mut probSlot: Ptr<CmptLzDecProb>;
    let mut probsMatrix: Ptr<CmptLzDecProb> = CmptLzGetProbsMatrix(decCtx.cast());
    let mut range: u32 = *pRange;
    let mut rangeCode: u32 = *pRangeCode;
    let mut rangeBound: u32 = *pRangeBound;
    probSlot = (CmptLzGetIsRepG0Prob(probsMatrix.cast()) + mkState).cast();
    if CMPTLZ_IS_THE_BIT_0!(probSlot, range, rangeCode, rangeBound).as_bool() {
        CMPTLZ_RANGE_UPDATE_0!(probSlot, range, rangeBound);
        CMPTLZ_RANGE_NORMALIZE!(range, rangeCode, bufToDec);
        probSlot = (CmptLzGetIsRepG0LongProb(probsMatrix.cast()) + posState + mkState).cast();
        if CMPTLZ_IS_THE_BIT_0!(probSlot, range, rangeCode, rangeBound).as_bool() {
            CMPTLZ_RANGE_UPDATE_0!(probSlot, range, rangeBound);
            CMPTLZ_RANGE_NORMALIZE!(range, rangeCode, bufToDec);
            *pRange = range.cast();
            *pRangeCode = rangeCode.cast();
            *pRangeBound = rangeBound.cast();
            decCtx.buf = bufToDec.cast();
            CmptLzShortRepDec(decCtx.cast());
            return CMPT_OK!();
        } else {
            CMPTLZ_RANGE_UPDATE_1!(probSlot, range, rangeCode, rangeBound);
            CMPTLZ_RANGE_NORMALIZE!(range, rangeCode, bufToDec);
            repDist = decCtx.reps[0].cast();
        }
    } else {
        CMPTLZ_RANGE_UPDATE_1!(probSlot, range, rangeCode, rangeBound);
        CMPTLZ_RANGE_NORMALIZE!(range, rangeCode, bufToDec);
        probSlot = (CmptLzGetIsRepG1Prob(probsMatrix.cast()) + mkState).cast();
        if CMPTLZ_IS_THE_BIT_0!(probSlot, range, rangeCode, rangeBound).as_bool() {
            CMPTLZ_RANGE_UPDATE_0!(probSlot, range, rangeBound);
            CMPTLZ_RANGE_NORMALIZE!(range, rangeCode, bufToDec);
            repDist = decCtx.reps[1].cast();
        } else {
            CMPTLZ_RANGE_UPDATE_1!(probSlot, range, rangeCode, rangeBound);
            CMPTLZ_RANGE_NORMALIZE!(range, rangeCode, bufToDec);
            probSlot = (CmptLzGetIsRepG2Prob(probsMatrix.cast()) + mkState).cast();
            if CMPTLZ_IS_THE_BIT_0!(probSlot, range, rangeCode, rangeBound).as_bool() {
                CMPTLZ_RANGE_UPDATE_0!(probSlot, range, rangeBound);
                CMPTLZ_RANGE_NORMALIZE!(range, rangeCode, bufToDec);
                repDist = decCtx.reps[CMPTLZ_REP2!()].cast();
            } else {
                CMPTLZ_RANGE_UPDATE_1!(probSlot, range, rangeCode, rangeBound);
                CMPTLZ_RANGE_NORMALIZE!(range, rangeCode, bufToDec);
                repDist = decCtx.reps[CMPTLZ_REP3!()].cast();
                decCtx.reps[CMPTLZ_REP3!()] = decCtx.reps[CMPTLZ_REP2!()].cast();
            }
            decCtx.reps[CMPTLZ_REP2!()] = decCtx.reps[1].cast();
        }
        decCtx.reps[1] = decCtx.reps[0].cast();
        decCtx.reps[0] = repDist.cast();
    }
    *pRange = range.cast();
    *pRangeCode = rangeCode.cast();
    *pRangeBound = rangeBound.cast();
    decCtx.buf = bufToDec.cast();
    decCtx.state = if mkState < CMPTLZ_LIT_STATES!() { 8 } else { 11 };
    probSlot = CmptLzGetRepLenCoderProb(probsMatrix.cast()).cast();
    repLen = CmptLzLenDec(decCtx.cast(), probSlot.cast(), pRange.cast(), pRangeCode.cast(), pRangeBound.cast(), posState.cast()).cast();
    return CmptLzDecByDistAndLen(decCtx.cast(), repDist.cast(), (repLen + 2).cast(), dicPosLimit.cast()).cast();
}
