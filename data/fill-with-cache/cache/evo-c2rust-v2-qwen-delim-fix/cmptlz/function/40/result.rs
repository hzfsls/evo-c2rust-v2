pub fn CmptLzTryDecOnePacket(mut decCtx: Ptr<CmptLzDecCtx>, mut bufTryDec: Ptr<u8>, mut pbufLimit: Ptr<Ptr<u8>>) -> i32 {
    let mut rangeBound: u32 = 0;
    let mut range: u32 = decCtx.range.cast();
    let mut rangeCode: u32 = decCtx.code.cast();
    let mut mkState: u32 = decCtx.state.cast();
    let mut bufLimit: Ptr<u8> = *pbufLimit.cast();
    let mut probSlot1: Ptr<CmptLzDecProb> = Default::default();
    let mut probSlot2: Ptr<CmptLzDecProb> = Default::default();
    let mut probsMatrix: Ptr<CmptLzDecProb> = CmptLzGetProbsMatrix(decCtx.cast()).cast();
    let mut pbMask: u32 = ((1 << decCtx.prop.posBits).cast::<u32>()) - 1;
    let mut posState: u32 = CMPTLZ_CALC_POS_STATE!(decCtx.processedPos, pbMask).cast();
    probSlot1 = CmptLzGetIsMatchProb(probsMatrix.cast()).cast() + posState + mkState;
    rangeBound = (range >> CMPTLZ_PROB_LG_BIT!().cast()) * (*probSlot1).cast();
    if (rangeCode < rangeBound).as_bool() {
        CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT0!(range, rangeBound);
        CMPTLZ_RANGE_TRY_NORMALIZE!(range, rangeCode, bufTryDec, bufLimit);
        return CmptLzTryDecLitPacket(decCtx.cast(), range.cast(), rangeCode.cast(), rangeBound.cast(), bufTryDec.cast(), pbufLimit.cast()).cast();
    }
    CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT1!(range, rangeCode, rangeBound);
    CMPTLZ_RANGE_TRY_NORMALIZE!(range, rangeCode, bufTryDec, bufLimit);
    probSlot2 = CmptLzGetIsRepProb(probsMatrix.cast()).cast() + mkState;
    rangeBound = (range >> CMPTLZ_PROB_LG_BIT!().cast()) * (*probSlot2).cast();
    if (rangeCode < rangeBound).as_bool() {
        CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT0!(range, rangeBound);
        let mut probSlot: Ptr<CmptLzDecProb> = CmptLzGetMatchLenCoderProb(probsMatrix.cast()).cast();
        mkState = 0;
    } else {
        if (decCtx.dictPos >= decCtx.dictBufSize).as_bool() {
            return CMPT_ERROR_DATA!();
        }
        CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT1!(range, rangeCode, rangeBound);
        CMPTLZ_RANGE_TRY_NORMALIZE!(range, rangeCode, bufTryDec, bufLimit);
        let mut probSlot: Ptr<CmptLzDecProb> = CmptLzGetIsRepG0Prob(probsMatrix.cast()).cast() + mkState;
        rangeBound = (range >> CMPTLZ_PROB_LG_BIT!().cast()) * (*probSlot).cast();
        if (rangeCode < rangeBound).as_bool() {
            CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT0!(range, rangeBound);
            CMPTLZ_RANGE_TRY_NORMALIZE!(range, rangeCode, bufTryDec, bufLimit);
            probSlot = CmptLzGetIsRepG0LongProb(probsMatrix.cast()).cast() + posState + mkState;
            rangeBound = (range >> CMPTLZ_PROB_LG_BIT!().cast()) * (*probSlot).cast();
            if (rangeCode < rangeBound).as_bool() {
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
            let mut probSlot: Ptr<CmptLzDecProb> = CmptLzGetIsRepG1Prob(probsMatrix.cast()).cast() + mkState;
            rangeBound = (range >> CMPTLZ_PROB_LG_BIT!().cast()) * (*probSlot).cast();
            if (rangeCode < rangeBound).as_bool() {
                CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT0!(range, rangeBound);
            } else {
                CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT1!(range, rangeCode, rangeBound);
                CMPTLZ_RANGE_TRY_NORMALIZE!(range, rangeCode, bufTryDec, bufLimit);
                let mut probSlot: Ptr<CmptLzDecProb> = CmptLzGetIsRepG2Prob(probsMatrix.cast()).cast() + mkState;
                rangeBound = (range >> CMPTLZ_PROB_LG_BIT!().cast()) * (*probSlot).cast();
                if (rangeCode < rangeBound).as_bool() {
                    CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT0!(range, rangeBound);
                } else {
                    CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT1!(range, rangeCode, rangeBound);
                }
            }
        }
        let mut probSlot: Ptr<CmptLzDecProb> = CmptLzGetRepLenCoderProb(probsMatrix.cast()).cast();
        mkState = CMPTLZ_MKSTATE_NUM!();
    }
    return CmptLzTryDecLenAndDist(decCtx.cast(), mkState.cast(), range.cast(), rangeCode.cast(), rangeBound.cast(), probSlot.cast(), bufTryDec.cast(), pbufLimit.cast()).cast();
}