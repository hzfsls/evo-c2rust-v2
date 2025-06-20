pub fn CmptLzTryDecLenAndDist(mut decCtx: Ptr<CmptLzDecCtx>, mut mkState: u32, mut range: u32, mut rangeCode: u32,
                              mut rangeBound: u32, mut probSlot: Ptr<CmptLzDecProb>, mut bufTryDec: Ptr<u8>,
                              mut pbufLimit: Ptr<Ptr<u8>>) -> i32 {
    let mut offset: u32;
    let mut bits2BeDec: u32;
    let mut pbMask: u32 = ((1 << decCtx.prop.posBits) - 1).cast();
    let mut posState: u32 = CMPTLZ_CALC_POS_STATE!(decCtx.processedPos, pbMask);
    let mut bufLimit: Ptr<u8> = *pbufLimit;
    let mut probBit: Ptr<CmptLzDecProb>;
    let mut probsMatrix: Ptr<CmptLzDecProb> = CmptLzGetProbsMatrix(decCtx);
    CMPTLZ_RANGE_TRY_NORMALIZE!(range, rangeCode, bufTryDec, bufLimit);
    let mut probLen: Ptr<CmptLzDecProb> = probSlot + CMPTLZ_LEN_CHOICE!();
    rangeBound = (range >> CMPTLZ_PROB_LG_BIT!()) * (*probLen as u32);
    if rangeCode < rangeBound {
        CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT0!(range, rangeBound);
        probLen = probSlot + CMPTLZ_LOW_LENPROB_OFFSET!() + posState;
        bits2BeDec = 3;
        offset = 0;
    } else {
        CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT1!(range, rangeCode, rangeBound);
        CMPTLZ_RANGE_TRY_NORMALIZE!(range, rangeCode, bufTryDec, bufLimit);
        probLen = probSlot + CMPTLZ_LEN_CHOICE2!();
        rangeBound = (range >> CMPTLZ_PROB_LG_BIT!()) * (*probLen as u32);
        if rangeCode < rangeBound {
            CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT0!(range, rangeBound);
            probLen = probSlot + CMPTLZ_LEN_CHOICE!() + CMPTLZ_LEN_CHOICE2!() + posState;
            bits2BeDec = 3;
            offset = CMPTLZ_LOW_LEN_CLASS!() << 1;
        } else {
            CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT1!(range, rangeCode, rangeBound);
            probLen = probSlot + CMPTLZ_HIGH_LENPROB_OFFSET!();
            bits2BeDec = 8;
            offset = CMPTLZ_LOW_LEN_CLASS!() << 1;
        }
    }
    CMPTLZ_RANGE_TRY_NORMALIZE!(range, rangeCode, bufTryDec, bufLimit);
    let mut decSym: u32 = 1;
    loop {
        probBit = probLen + decSym;
        CMPTLZ_SINGLE_BIT_TRY_DEC!(range, rangeCode, rangeBound, decSym, probBit);
        CMPTLZ_RANGE_TRY_NORMALIZE!(range, rangeCode, bufTryDec, bufLimit);
        if decSym >= (1 << bits2BeDec) {
            break;
        }
    }
    decSym -= 1 << bits2BeDec;
    decSym += offset;
    if mkState >= 4 {
        *pbufLimit = bufTryDec;
        return CMPT_OK!();
    }
    probSlot = CmptLzGetPosSlotProb(probsMatrix) + CmptLzGetLenCondition(decSym);
    decSym = 1;
    loop {
        probBit = probSlot + decSym;
        CMPTLZ_SINGLE_BIT_TRY_DEC!(range, rangeCode, rangeBound, decSym, probBit);
        CMPTLZ_RANGE_TRY_NORMALIZE!(range, rangeCode, bufTryDec, bufLimit);
        if decSym >= (1 << CMPTLZ_POS_SLOT_BITS!()) {
            break;
        }
    }
    decSym -= 1 << CMPTLZ_POS_SLOT_BITS!();
    bits2BeDec = (decSym >> 1) - 1;
    if decSym >= CMPTLZ_LOW_POSSLOT!() {
        if decSym < CMPTLZ_HIGH_POSSLOT!() {
            probSlot = CmptLzGetSpecPosProb(probsMatrix) + (CmptLzGetBaseDistByPosSlot(decSym) << bits2BeDec);
        } else {
            bits2BeDec -= CMPTLZ_LARGE_DIST_LOW_BITS!();
            loop {
                CMPTLZ_RANGE_TRY_NORMALIZE!(range, rangeCode, bufTryDec, bufLimit);
                range >>= 1;
                rangeCode -= range & (((rangeCode - range) >> 31) - 1);
                bits2BeDec -= 1;
                if bits2BeDec == 0 {
                    break;
                }                
            }
            probSlot = CmptLzGetAilgnProb(probsMatrix);
            bits2BeDec = CMPTLZ_LARGE_DIST_LOW_BITS!();
        }
        decSym = 1;
        offset = 1;
        loop {
            CMPTLZ_RANGE_TRY_NORMALIZE!(range, rangeCode, bufTryDec, bufLimit);
            probBit = probSlot + decSym;
            rangeBound = (range >> CMPTLZ_PROB_LG_BIT!()) * (*probBit as u32);
            if rangeCode < rangeBound {
                CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT0!(range, rangeBound);
                decSym += offset;
                offset <<= 1;
            } else {
                CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT1!(range, rangeCode, rangeBound);
                offset <<= 1;
                decSym += offset;
            }
            bits2BeDec -= 1;
            if bits2BeDec == 0 {
                break;
            }            
        }
    }
    CMPTLZ_RANGE_TRY_NORMALIZE!(range, rangeCode, bufTryDec, bufLimit);
    *pbufLimit = bufTryDec;
    return CMPT_OK!();
}