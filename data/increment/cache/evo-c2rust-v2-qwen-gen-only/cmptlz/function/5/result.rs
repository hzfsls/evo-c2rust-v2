pub fn CmptLzTryDecLenAndDist(mut decCtx: Ptr<CmptLzDecCtx>, mut mkState: u32, mut range: u32, mut rangeCode: u32, mut rangeBound: u32, mut probSlot: Ptr<CmptLzDecProb>, mut bufTryDec: Ptr<u8>, mut pbufLimit: Ptr<Ptr<u8>>) -> i32 {
    let mut offset: u32 = Default::default();
    let mut bits2BeDec: u32 = Default::default();
    let mut pbMask: u32 = ((1 << decCtx.prop.posBits) - 1).cast();
    let mut posState: u32 = CMPTLZ_CALC_POS_STATE!(decCtx.processedPos, pbMask).cast();
    let mut bufLimit: Ptr<u8> = *pbufLimit.cast();
    let mut probBit: Ptr<CmptLzDecProb> = Default::default();
    let mut probsMatrix: Ptr<CmptLzDecProb> = CmptLzGetProbsMatrix(decCtx.cast()).cast();

    CMPTLZ_RANGE_TRY_NORMALIZE!(range, rangeCode, bufTryDec, bufLimit);
    let mut probLen: Ptr<CmptLzDecProb> = probSlot + CMPTLZ_LEN_CHOICE;
    rangeBound = (range >> CMPTLZ_PROB_LG_BIT) * (*probLen).cast();
    if (rangeCode < rangeBound).as_bool() {
        CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT0!(range, rangeBound);
        probLen = probSlot + CMPTLZ_LOW_LENPROB_OFFSET + posState;
        bits2BeDec = 3;
        offset = 0;
    } else {
        CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT1!(range, rangeCode, rangeBound);
        CMPTLZ_RANGE_TRY_NORMALIZE!(range, rangeCode, bufTryDec, bufLimit);

        probLen = probSlot + CMPTLZ_LEN_CHOICE2;
        rangeBound = (range >> CMPTLZ_PROB_LG_BIT) * (*probLen).cast();
        if (rangeCode < rangeBound).as_bool() {
            CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT0!(range, rangeBound);
            probLen = probSlot + CMPTLZ_LEN_CHOICE + CMPTLZ_LEN_CHOICE2 + posState;
            bits2BeDec = 3;
            offset = (CMPTLZ_LOW_LEN_CLASS << 1).cast();
        } else {
            CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT1!(range, rangeCode, rangeBound);
            probLen = probSlot + CMPTLZ_HIGH_LENPROB_OFFSET;
            bits2BeDec = 8;
            offset = (CMPTLZ_LOW_LEN_CLASS << 1).cast();
        }
    }

    CMPTLZ_RANGE_TRY_NORMALIZE!(range, rangeCode, bufTryDec, bufLimit);

    let mut decSym: u32 = 1;
    loop {
        probBit = probLen + decSym;
        CMPTLZ_SINGLE_BIT_TRY_DEC!(range, rangeCode, rangeBound, decSym, probBit);
        CMPTLZ_RANGE_TRY_NORMALIZE!(range, rangeCode, bufTryDec, bufLimit);
        if (decSym < ((1 << bits2BeDec)).cast()).as_bool() {
            break;
        }
    }
    decSym -= (1 << bits2BeDec).cast();
    decSym += offset;

    if (mkState >= 4).as_bool() {
        *pbufLimit = bufTryDec.cast();
        return CMPT_OK!();
    }

    probSlot = CmptLzGetPosSlotProb(probsMatrix).cast() + CmptLzGetLenCondition(decSym).cast();

    decSym = 1;
    loop {
        probBit = probSlot + decSym;
        CMPTLZ_SINGLE_BIT_TRY_DEC!(range, rangeCode, rangeBound, decSym, probBit);
        CMPTLZ_RANGE_TRY_NORMALIZE!(range, rangeCode, bufTryDec, bufLimit);
        if (decSym < (1 << CMPTLZ_POS_SLOT_BITS).cast()).as_bool() {
            break;
        }
    }
    decSym -= (1 << CMPTLZ_POS_SLOT_BITS).cast();

    bits2BeDec = ((decSym >> 1) - 1).cast();
    if (decSym >= CMPTLZ_LOW_POSSLOT!()).as_bool() {
        if (decSym < CMPTLZ_HIGH_POSSLOT!()).as_bool() {
            probSlot = CmptLzGetSpecPosProb(probsMatrix).cast() + (CmptLzGetBaseDistByPosSlot(decSym).cast() << bits2BeDec);
        } else {
            bits2BeDec -= CMPTLZ_LARGE_DIST_LOW_BITS!();
            loop {
                CMPTLZ_RANGE_TRY_NORMALIZE!(range, rangeCode, bufTryDec, bufLimit);
                range >>= 1;
                rangeCode -= range & (((rangeCode - range) >> 31) - 1).cast();
                if (bits2BeDec == 0) {
                    break;
                }
                bits2BeDec -= 1;
            }
            probSlot = CmptLzGetAilgnProb(probsMatrix).cast();
            bits2BeDec = CMPTLZ_LARGE_DIST_LOW_BITS!();
        }

        decSym = 1;
        offset = 1;
        loop {
            CMPTLZ_RANGE_TRY_NORMALIZE!(range, rangeCode, bufTryDec, bufLimit);
            probBit = probSlot + decSym;
            rangeBound = (range >> CMPTLZ_PROB_LG_BIT) * (*probBit).cast();
            if (rangeCode < rangeBound).as_bool() {
                CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT0!(range, rangeBound);
                decSym += offset;
                offset <<= 1;
            } else {
                CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT1!(range, rangeCode, rangeBound);
                offset <<= 1;
                decSym += offset;
            }
            if (bits2BeDec == 0) {
                break;
            }
            bits2BeDec -= 1;
        }
    }

    CMPTLZ_RANGE_TRY_NORMALIZE!(range, rangeCode, bufTryDec, bufLimit);
    *pbufLimit = bufTryDec.cast();
    return CMPT_OK!();
}