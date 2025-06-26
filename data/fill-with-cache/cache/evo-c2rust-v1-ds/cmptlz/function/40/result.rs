pub fn CmptLzTryDecOnePacket(mut decCtx: Ptr<CmptLzDecCtx>, mut bufTryDec: Ptr<u8>, mut pbufLimit: Ptr<Ptr<u8>>) -> i32 {
    let mut rangeBound: u32 = 0;
    let mut range: u32 = decCtx.range.cast();
    let mut rangeCode: u32 = decCtx.code.cast();
    let mut mkState: u32 = decCtx.state.cast();
    let mut bufLimit: Ptr<u8> = (*pbufLimit).cast();
    let mut probSlot: Ptr<CmptLzDecProb> = Default::default();
    let mut probSlot1: Ptr<CmptLzDecProb> = Default::default();
    let mut probSlot2: Ptr<CmptLzDecProb> = Default::default();
    let mut probsMatrix: Ptr<CmptLzDecProb> = CmptLzGetProbsMatrix(decCtx.cast());
    let mut pbMask: u32 = ((1).cast::<u32>() << decCtx.prop.posBits) - 1;
    let mut posState: u32 = CMPTLZ_CALC_POS_STATE!(decCtx.processedPos, pbMask);
    probSlot1 = (CmptLzGetIsMatchProb(probsMatrix.cast()) + posState + mkState).cast();
    rangeBound = (range >> CMPTLZ_PROB_LG_BIT!()) * (*probSlot1).cast::<u32>();
    if rangeCode < rangeBound {
        CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT0!(range, rangeBound);
        CMPTLZ_RANGE_TRY_NORMALIZE!(range, rangeCode, bufTryDec, bufLimit);
        return CmptLzTryDecLitPacket(decCtx.cast(), range.cast(), rangeCode.cast(), rangeBound.cast(), bufTryDec.cast(), pbufLimit.cast()).cast();
    }
    CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT1!(range, rangeCode, rangeBound);
    CMPTLZ_RANGE_TRY_NORMALIZE!(range, rangeCode, bufTryDec, bufLimit);
    probSlot2 = (CmptLzGetIsRepProb(probsMatrix.cast()) + mkState).cast();
    rangeBound = (range >> CMPTLZ_PROB_LG_BIT!()) * (*probSlot2).cast::<u32>();
    if rangeCode < rangeBound {
        CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT0!(range, rangeBound);
        probSlot = CmptLzGetMatchLenCoderProb(probsMatrix.cast()).cast();
        mkState = 0;
    } else {
        if decCtx.dictPos >= decCtx.dictBufSize {
            return CMPT_ERROR_DATA!();
        }
        CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT1!(range, rangeCode, rangeBound);
        CMPTLZ_RANGE_TRY_NORMALIZE!(range, rangeCode, bufTryDec, bufLimit);
        probSlot = (CmptLzGetIsRepG0Prob(probsMatrix.cast()) + mkState).cast();
        rangeBound = (range >> CMPTLZ_PROB_LG_BIT!()) * (*probSlot).cast::<u32>();
        if rangeCode < rangeBound {
            CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT0!(range, rangeBound);
            CMPTLZ_RANGE_TRY_NORMALIZE!(range, rangeCode, bufTryDec, bufLimit);
            probSlot = (CmptLzGetIsRepG0LongProb(probsMatrix.cast()) + posState + mkState).cast();
            rangeBound = (range >> CMPTLZ_PROB_LG_BIT!()) * (*probSlot).cast::<u32>();
            if rangeCode < rangeBound {
                CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT0!(range, rangeBound);
                CMPTLZ_RANGE_TRY_NORMALIZE!(range, rangeCode, bufTryDec, bufLimit);
                *pbufLimit = bufTryDec.cast();
                return CMPT_OK!();
            } else {
                CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT1!(range, rangeCode, rangeBound);
            }
        } else {
            CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT1!(range, rangeCode, rangeBound);
            CMPTLZ_RANGE_TRY_NORMALIZE!(range, rangeCode, bufTryDec, bufLimit);
            probSlot = (CmptLzGetIsRepG1Prob(probsMatrix.cast()) + mkState).cast();
            rangeBound = (range >> CMPTLZ_PROB_LG_BIT!()) * (*probSlot).cast::<u32>();
            if rangeCode < rangeBound {
                CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT0!(range, rangeBound);
            } else {
                CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT1!(range, rangeCode, rangeBound);
                CMPTLZ_RANGE_TRY_NORMALIZE!(range, rangeCode, bufTryDec, bufLimit);
                probSlot = (CmptLzGetIsRepG2Prob(probsMatrix.cast()) + mkState).cast();
                rangeBound = (range >> CMPTLZ_PROB_LG_BIT!()) * (*probSlot).cast::<u32>();
                if rangeCode < rangeBound {
                    CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT0!(range, rangeBound);
                } else {
                    CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT1!(range, rangeCode, rangeBound);
                }
            }
        }
        probSlot = CmptLzGetRepLenCoderProb(probsMatrix.cast()).cast();
        mkState = CMPTLZ_MKSTATE_NUM!();
    }
    return CmptLzTryDecLenAndDist(decCtx.cast(), mkState.cast(), range.cast(), rangeCode.cast(), rangeBound.cast(), probSlot.cast(), bufTryDec.cast(), pbufLimit.cast()).cast();
}
