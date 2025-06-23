pub fn CmptLzTryDecLenAndDist(mut decCtx: Ptr<CmptLzDecCtx>, mut mkState: u32, mut range: u32, mut rangeCode: u32, mut rangeBound: u32, mut probSlot: Ptr<CmptLzDecProb>, mut bufTryDec: Ptr<u8>, mut pbufLimit: Ptr<Ptr<u8>>) -> i32 {
    let mut offset: u32;
    let mut bits2BeDec: u32;
    let mut pbMask: u32 = ((1 as u32) << decCtx.prop.posBits) - 1;
    let mut posState: u32 = CMPTLZ_CALC_POS_STATE!(decCtx.processedPos, pbMask);
    let mut bufLimit: Ptr<u8> = *pbufLimit;
    let mut probBit: Ptr<CmptLzDecProb>;
    let mut probsMatrix: Ptr<CmptLzDecProb> = CmptLzGetProbsMatrix(decCtx.cast());

    CMPTLZ_RANGE_TRY_NORMALIZE!(range, rangeCode, bufTryDec, bufLimit);
    let mut probLen: Ptr<CmptLzDecProb> = probSlot + CMPTLZ_LEN_CHOICE!();
    rangeBound = (range >> CMPTLZ_PROB_LG_BIT!()) * (*probLen);
    if (rangeCode < rangeBound).as_bool() {
        CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT0!(range, rangeBound);
        probLen = probSlot + CMPTLZ_LOW_LENPROB_OFFSET!() + posState;
        bits2BeDec = 3;
        offset = 0;
    } else {
        CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT1!(range, rangeCode, rangeBound);
        CMPTLZ_RANGE_TRY_NORMALIZE!(range, rangeCode, bufTryDec, bufLimit);

        probLen = probSlot + CMPTLZ_LEN_CHOICE2!();
        rangeBound = (range >> CMPTLZ_PROB_LG_BIT!()) * (*probLen);
        if (rangeCode < rangeBound).as_bool() {
            CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT0!(range, rangeBound);
            probLen = probSlot + CMPTLZ_LEN_CHOICE!() + CMPTLZ_LEN_CHOICE2!() + posState;
            bits2BeDec = 3;
            offset = (CMPTLZ_LOW_LEN_CLASS!() << 1);
        } else {
            CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT1!(range, rangeCode, rangeBound);
            probLen = probSlot + CMPTLZ_HIGH_LENPROB_OFFSET!();
            bits2BeDec = 8;
            offset = (CMPTLZ_LOW_LEN_CLASS!() << 1);
        }
    }

    CMPTLZ_RANGE_TRY_NORMALIZE!(range, rangeCode, bufTryDec, bufLimit);

    let mut decSym: u32 = 1;
    c_do!({
        probBit = probLen + decSym;
        CMPTLZ_SINGLE_BIT_TRY_DEC!(range, rangeCode, rangeBound, decSym, probBit);
        CMPTLZ_RANGE_TRY_NORMALIZE!(range, rangeCode, bufTryDec, bufLimit);
    } while decSym < ((1 as u32) << bits2BeDec));
    decSym -= ((1 as u32) << bits2BeDec);
    decSym += offset;

    if (mkState >= 4).as_bool() {
        *pbufLimit = bufTryDec.cast();
        return CMPT_OK!();
    }

    probSlot = CmptLzGetPosSlotProb(probsMatrix.cast()) + CmptLzGetLenCondition(decSym.cast());

    decSym = 1;
    c_do!({
        probBit = probSlot + decSym;
        CMPTLZ_SINGLE_BIT_TRY_DEC!(range, rangeCode, rangeBound, decSym, probBit);
        CMPTLZ_RANGE_TRY_NORMALIZE!(range, rangeCode, bufTryDec, bufLimit);
    } while decSym < (1 << CMPTLZ_POS_SLOT_BITS!()));
    decSym -= (1 << CMPTLZ_POS_SLOT_BITS!());

    bits2BeDec = ((decSym >> 1) - 1);
    if (decSym >= CMPTLZ_LOW_POSSLOT!()).as_bool() {
        if (decSym < CMPTLZ_HIGH_POSSLOT!()).as_bool() {
            probSlot = CmptLzGetSpecPosProb(probsMatrix.cast()) + (CmptLzGetBaseDistByPosSlot(decSym.cast()) << bits2BeDec);
        } else {
            bits2BeDec -= CMPTLZ_LARGE_DIST_LOW_BITS!();
            c_do!({
                CMPTLZ_RANGE_TRY_NORMALIZE!(range, rangeCode, bufTryDec, bufLimit);
                range >>= 1;
                rangeCode -= range & (((rangeCode - range) >> 31) - 1);
            } while bits2BeDec.prefix_minus_minus() > 0);
            probSlot = CmptLzGetAilgnProb(probsMatrix.cast());
            bits2BeDec = CMPTLZ_LARGE_DIST_LOW_BITS!();
        }

        decSym = 1;
        offset = 1;
        c_do!({
            CMPTLZ_RANGE_TRY_NORMALIZE!(range, rangeCode, bufTryDec, bufLimit);
            probBit = probSlot + decSym;
            rangeBound = (range >> CMPTLZ_PROB_LG_BIT!()) * (*probBit);
            if (rangeCode < rangeBound).as_bool() {
                CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT0!(range, rangeBound);
                decSym += offset;
                offset <<= 1;
            } else {
                CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT1!(range, rangeCode, rangeBound);
                offset <<= 1;
                decSym += offset;
            }
        } while bits2BeDec.prefix_minus_minus() > 0);
    }

    CMPTLZ_RANGE_TRY_NORMALIZE!(range, rangeCode, bufTryDec, bufLimit);
    *pbufLimit = bufTryDec.cast();
    return CMPT_OK!();
}
