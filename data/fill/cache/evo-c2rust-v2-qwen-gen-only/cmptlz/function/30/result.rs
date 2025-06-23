pub fn CmptLzDistDec(mut decCtx: Ptr<CmptLzDecCtx>, mut probsMatrix: Ptr<CmptLzDecProb>, mut pRange: Ptr<u32>, mut pRangeCode: Ptr<u32>, mut pRangeBound: Ptr<u32>, mut decLen: u32) -> usize {
    let mut assistBits: u32 = Default::default();
    let mut posSlot: u32 = 1;
    let mut range: u32 = *pRange;
    let mut rangeCode: u32 = *pRangeCode;
    let mut rangeBound: u32 = *pRangeBound;
    let mut bufToDec: Ptr<u8> = decCtx.buf.cast();
    let mut distDec: u32 = Default::default();
    let mut probPosSlot: Ptr<CmptLzDecProb> = CmptLzGetPosSlotProb(probsMatrix).cast() + CmptLzGetLenCondition(decLen).cast();
    let mut i: i32 = 0;
    c_for!(i = 0; i < CMPTLZ_POS_SLOT_BITS!(); i.suffix_plus_plus(); {
        CMPTLZ_POSSLOT_BIT_DEC((probPosSlot + posSlot).cast(), range.cast(), rangeCode.cast(), rangeBound.cast(), posSlot.cast(), bufToDec.cast());
    });
    posSlot -= 64;
    if (posSlot < CMPTLZ_LOW_POSSLOT!()).as_bool() {
        distDec = posSlot;
        CmptLzDistDecHelper(decCtx.cast(), distDec.cast(), bufToDec.cast(), pRange.cast(), pRangeCode.cast(), pRangeBound.cast(), range.cast(), rangeCode.cast(), rangeBound.cast());
        if (distDec == 0xFFFFFFFF).as_bool() {
            return distDec.cast();
        } else {
            return (distDec + 1).cast();
        }
    }
    let mut directBitNum: u32 = ((posSlot >> 1) - 1);
    distDec = CmptLzGetBaseDistByPosSlot(posSlot).cast();
    if (posSlot < CMPTLZ_HIGH_POSSLOT!()).as_bool() {
        assistBits = 1;
        distDec <<= directBitNum;
        distDec += assistBits;
        probPosSlot = CmptLzGetSpecPosProb(probsMatrix).cast();
        c_do!({
            if CMPTLZ_IS_THE_BIT_0!((probPosSlot + distDec).cast(), range.cast(), rangeCode.cast(), rangeBound.cast()).as_bool() {
                CMPTLZ_RANGE_UPDATE_0!((probPosSlot + distDec).cast(), range.cast(), rangeBound.cast());
                CMPTLZ_RANGE_NORMALIZE!(range.cast(), rangeCode.cast(), bufToDec.cast());
                distDec += assistBits;
                assistBits <<= 1;
            } else {
                CMPTLZ_RANGE_UPDATE_1!((probPosSlot + distDec).cast(), range.cast(), rangeCode.cast(), rangeBound.cast());
                CMPTLZ_RANGE_NORMALIZE!(range.cast(), rangeCode.cast(), bufToDec.cast());
                assistBits <<= 1;
                distDec += assistBits;
            }
        } while directBitNum.suffix_minus_minus() > 0);
        distDec -= assistBits;
    } else {
        directBitNum -= CMPTLZ_REP4!();
        c_do!({
            CMPTLZ_RANGE_NORMALIZE!(range.cast(), rangeCode.cast(), bufToDec.cast());
            range >>= 1;
            rangeCode -= range;
            assistBits = (0 - ((rangeCode >> 31).cast::<u32>()));
            distDec = (distDec << 1) + (assistBits + 1);
            rangeCode += range & assistBits;
        } while directBitNum.suffix_minus_minus() > 0);
        let mut probDist: Ptr<CmptLzDecProb> = Default::default();
        probPosSlot = CmptLzGetAilgnProb(probsMatrix).cast();
        distDec <<= CMPTLZ_LARGE_DIST_LOW_BITS!();
        assistBits = 1;
        let mut cycleSym: u32 = 1;
        c_for!(i = 0; i < 3; i.suffix_plus_plus(); {
            CMPTLZ_RANGE_NORMALIZE!(range.cast(), rangeCode.cast(), bufToDec.cast());
            CMPTLZ_DIST_BIT_DEC!(probDist.cast(), probPosSlot.cast(), range.cast(), rangeCode.cast(), rangeBound.cast(), assistBits.cast(), cycleSym.cast());
            cycleSym <<= 1;
        });
        CMPTLZ_RANGE_NORMALIZE!(range.cast(), rangeCode.cast(), bufToDec.cast());
        probDist = probPosSlot + assistBits;
        rangeBound = (range >> CMPTLZ_PROB_LG_BIT!()) * (*probDist);
        if (rangeCode < rangeBound).as_bool() {
            CMPTLZ_RANGE_UPDATE_0!(probDist.cast(), range.cast(), rangeBound.cast());
            assistBits -= 8;
        } else {
            CMPTLZ_RANGE_UPDATE_1!(probDist.cast(), range.cast(), rangeCode.cast(), rangeBound.cast());
        }
        CMPTLZ_RANGE_NORMALIZE!(range.cast(), rangeCode.cast(), bufToDec.cast());
        distDec |= assistBits;
    }
    CmptLzDistDecHelper(decCtx.cast(), distDec.cast(), bufToDec.cast(), pRange.cast(), pRangeCode.cast(), pRangeBound.cast(), range.cast(), rangeCode.cast(), rangeBound.cast());
    if (distDec == 0xFFFFFFFF).as_bool() {
        return distDec.cast();
    } else {
        return (distDec + 1).cast();
    }
}