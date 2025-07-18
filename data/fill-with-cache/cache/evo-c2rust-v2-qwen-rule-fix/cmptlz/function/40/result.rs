pub fn CmptLzTryDecOnePacket(mut decCtx: Ptr<CmptLzDecCtx>, mut bufTryDec: Ptr<u8>, mut pbufLimit: Ptr<Ptr<u8>>) -> i32 {
    let mut rangeBound: u32 = 0;
    let mut range: u32 = decCtx.range;
    let mut rangeCode: u32 = decCtx.code;
    let mut mkState: u32 = decCtx.state;
    let mut bufLimit: Ptr<u8> = *pbufLimit;
    let mut probSlot1: Ptr<CmptLzDecProb> = Default::default();
    let mut probSlot2: Ptr<CmptLzDecProb> = Default::default();
    let mut probsMatrix: Ptr<CmptLzDecProb> = CmptLzGetProbsMatrix(decCtx);
    let mut pbMask: u32 = ((1 << decCtx.prop.posBits).cast::<u32>()) - 1;
    let mut posState: u32 = CMPTLZ_CALC_POS_STATE!(decCtx.processedPos, pbMask);
    probSlot1 = CmptLzGetIsMatchProb(probsMatrix) + posState + mkState;
    rangeBound = (range >> CMPTLZ_PROB_LG_BIT!()) * (*probSlot1).cast();
    if (rangeCode < rangeBound) {
        CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT0!(range, rangeBound);
        CMPTLZ_RANGE_TRY_NORMALIZE!(range, rangeCode, bufTryDec, bufLimit);
        return CmptLzTryDecLitPacket(decCtx, range, rangeCode, rangeBound, bufTryDec, pbufLimit);
    }
    CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT1!(range, rangeCode, rangeBound);
    CMPTLZ_RANGE_TRY_NORMALIZE!(range, rangeCode, bufTryDec, bufLimit);
    probSlot2 = CmptLzGetIsRepProb(probsMatrix) + mkState;
    rangeBound = (range >> CMPTLZ_PROB_LG_BIT!()) * (*probSlot2).cast();
    if (rangeCode < rangeBound) {
        CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT0!(range, rangeBound);
        let mut probSlot: Ptr<CmptLzDecProb> = CmptLzGetMatchLenCoderProb(probsMatrix);
        mkState = 0;
    } else {
        if (decCtx.dictPos >= decCtx.dictBufSize) {
            return CMPT_ERROR_DATA!();
        }
        CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT1!(range, rangeCode, rangeBound);
        CMPTLZ_RANGE_TRY_NORMALIZE!(range, rangeCode, bufTryDec, bufLimit);
        let mut probSlot: Ptr<CmptLzDecProb> = CmptLzGetIsRepG0Prob(probsMatrix) + mkState;
        rangeBound = (range >> CMPTLZ_PROB_LG_BIT!()) * (*probSlot).cast();
        if (rangeCode < rangeBound) {
            CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT0!(range, rangeBound);
            CMPTLZ_RANGE_TRY_NORMALIZE!(range, rangeCode, bufTryDec, bufLimit);
            probSlot = CmptLzGetIsRepG0LongProb(probsMatrix) + posState + mkState;
            rangeBound = (range >> CMPTLZ_PROB_LG_BIT!()) * (*probSlot).cast();
            if (rangeCode < rangeBound) {
                CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT0!(range, rangeBound);
                CMPTLZ_RANGE_TRY_NORMALIZE!(range, rangeCode, bufTryDec, bufLimit);
                *pbufLimit = bufTryDec;
                return CMPT_OK!();
            } else {
                CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT1!(range, rangeCode, rangeBound);
            }
        } else {
            CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT1!(range, rangeCode, rangeBound);
            CMPTLZ_RANGE_TRY_NORMALIZE!(range, rangeCode, bufTryDec, bufLimit);
            let mut probSlot: Ptr<CmptLzDecProb> = CmptLzGetIsRepG1Prob(probsMatrix) + mkState;
            rangeBound = (range >> CMPTLZ_PROB_LG_BIT!()) * (*probSlot).cast();
            if (rangeCode < rangeBound) {
                CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT0!(range, rangeBound);
            } else {
                CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT1!(range, rangeCode, rangeBound);
                CMPTLZ_RANGE_TRY_NORMALIZE!(range, rangeCode, bufTryDec, bufLimit);
                let mut probSlot: Ptr<CmptLzDecProb> = CmptLzGetIsRepG2Prob(probsMatrix) + mkState;
                rangeBound = (range >> CMPTLZ_PROB_LG_BIT!()) * (*probSlot).cast();
                if (rangeCode < rangeBound) {
                    CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT0!(range, rangeBound);
                } else {
                    CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT1!(range, rangeCode, rangeBound);
                }
            }
        }
        let mut probSlot: Ptr<CmptLzDecProb> = CmptLzGetRepLenCoderProb(probsMatrix);
        mkState = CMPTLZ_MKSTATE_NUM!();
    }
    return CmptLzTryDecLenAndDist(decCtx, mkState, range, rangeCode, rangeBound, probSlot, bufTryDec, pbufLimit);
}