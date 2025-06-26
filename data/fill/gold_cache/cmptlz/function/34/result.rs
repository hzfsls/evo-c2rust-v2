pub fn CmptLzRepDec(mut decCtx: Ptr<CmptLzDecCtx>, mut pRange: Ptr<u32>, mut pRangeCode: Ptr<u32>, mut pRangeBound: Ptr<u32>,
                    dicPosLimit: usize, posState: u32) -> u32 {
    let mut repLen: u32;
    let mut repDist: u32;
    let mut mkState: u32 = decCtx.state;
    let mut bufToDec: Ptr<u8> = decCtx.buf;
    let mut probSlot: Ptr<CmptLzDecProb>;
    let mut probsMatrix: Ptr<CmptLzDecProb> = CmptLzGetProbsMatrix(decCtx);
    let mut range: u32 = *pRange;
    let mut rangeCode: u32 = *pRangeCode;
    let mut rangeBound: u32 = *pRangeBound;
    probSlot = CmptLzGetIsRepG0Prob(probsMatrix) + mkState;
    if CMPTLZ_IS_THE_BIT_0!(probSlot, range, rangeCode, rangeBound) {
        CMPTLZ_RANGE_UPDATE_0!(probSlot, range, rangeBound);
        CMPTLZ_RANGE_NORMALIZE!(range, rangeCode, bufToDec);
        probSlot = CmptLzGetIsRepG0LongProb(probsMatrix) + posState + mkState;
        if CMPTLZ_IS_THE_BIT_0!(probSlot, range, rangeCode, rangeBound) {
            CMPTLZ_RANGE_UPDATE_0!(probSlot, range, rangeBound);
            CMPTLZ_RANGE_NORMALIZE!(range, rangeCode, bufToDec);
            *pRange = range;
            *pRangeCode = rangeCode;
            *pRangeBound = rangeBound;
            decCtx.buf = bufToDec;
            CmptLzShortRepDec(decCtx);
            return CMPT_OK!();
        } else {
            CMPTLZ_RANGE_UPDATE_1!(probSlot, range, rangeCode, rangeBound);
            CMPTLZ_RANGE_NORMALIZE!(range, rangeCode, bufToDec);
            repDist = decCtx.reps[0];
        }
    } else {
        CMPTLZ_RANGE_UPDATE_1!(probSlot, range, rangeCode, rangeBound);
        CMPTLZ_RANGE_NORMALIZE!(range, rangeCode, bufToDec);
        probSlot = CmptLzGetIsRepG1Prob(probsMatrix) + mkState;
        if CMPTLZ_IS_THE_BIT_0!(probSlot, range, rangeCode, rangeBound) {
            CMPTLZ_RANGE_UPDATE_0!(probSlot, range, rangeBound);
            CMPTLZ_RANGE_NORMALIZE!(range, rangeCode, bufToDec);
            repDist = decCtx.reps[1];
        } else {
            CMPTLZ_RANGE_UPDATE_1!(probSlot, range, rangeCode, rangeBound);
            CMPTLZ_RANGE_NORMALIZE!(range, rangeCode, bufToDec);
            probSlot = CmptLzGetIsRepG2Prob(probsMatrix) + mkState;
            if CMPTLZ_IS_THE_BIT_0!(probSlot, range, rangeCode, rangeBound) {
                CMPTLZ_RANGE_UPDATE_0!(probSlot, range, rangeBound);
                CMPTLZ_RANGE_NORMALIZE!(range, rangeCode, bufToDec);
                repDist = decCtx.reps[CMPTLZ_REP2!()];
            } else {
                CMPTLZ_RANGE_UPDATE_1!(probSlot, range, rangeCode, rangeBound);
                CMPTLZ_RANGE_NORMALIZE!(range, rangeCode, bufToDec);
                repDist = decCtx.reps[CMPTLZ_REP3!()];
                decCtx.reps[CMPTLZ_REP3!()] = decCtx.reps[CMPTLZ_REP2!()];
            }
            decCtx.reps[CMPTLZ_REP2!()] = decCtx.reps[1];
        }
        decCtx.reps[1] = decCtx.reps[0];
        decCtx.reps[0] = repDist;
    }
    *pRange = range;
    *pRangeCode = rangeCode;
    *pRangeBound = rangeBound;
    decCtx.buf = bufToDec;
    decCtx.state = if mkState < CMPTLZ_LIT_STATES!() { 8 } else { 11 };
    probSlot = CmptLzGetRepLenCoderProb(probsMatrix);
    repLen = CmptLzLenDec(decCtx, probSlot, pRange, pRangeCode, pRangeBound, posState);
    return CmptLzDecByDistAndLen(decCtx, repDist as usize, repLen + 2, dicPosLimit);
}