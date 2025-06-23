pub fn CmptLzLenDec(mut decCtx: Ptr<CmptLzDecCtx>, mut probSlot: Ptr<CmptLzDecProb>, mut pRange: Ptr<u32>, mut pRangeCode: Ptr<u32>, mut pRangeBound: Ptr<u32>, mut posState: u32) -> u32 {
    let mut decLen: u32 = 1;
    let mut range: u32 = *pRange;
    let mut rangeCode: u32 = *pRangeCode;
    let mut rangeBound: u32 = *pRangeBound;
    let mut bufToDec: Ptr<u8> = decCtx.buf.cast();
    let mut probLen: Ptr<CmptLzDecProb> = (probSlot + CMPTLZ_LEN_CHOICE!()).cast();

    let mut i: i32 = 0;
    if CMPTLZ_IS_THE_BIT_0!(probLen, range, rangeCode, rangeBound).as_bool() {
        CMPTLZ_RANGE_UPDATE_0!(probLen, range, rangeBound);
        CMPTLZ_RANGE_NORMALIZE!(range, rangeCode, bufToDec);
        probLen = (probSlot + CMPTLZ_LOW_LENPROB_OFFSET!() + posState).cast();
        c_for!(i = 0; i < CMPTLZ_LOW_LEN_BIT!().cast(); i.suffix_plus_plus(); {
            CMPTLZ_LEN_BIT_DEC!((probLen + decLen), range, rangeCode, rangeBound, decLen, bufToDec);
        });
        decLen -= 8;
    } else {
        CMPTLZ_RANGE_UPDATE_1!(probLen, range, rangeCode, rangeBound);
        CMPTLZ_RANGE_NORMALIZE!(range, rangeCode, bufToDec);

        probLen = (probSlot + CMPTLZ_LEN_CHOICE2!()).cast();
        if CMPTLZ_IS_THE_BIT_0!(probLen, range, rangeCode, rangeBound).as_bool() {
            CMPTLZ_RANGE_UPDATE_0!(probLen, range, rangeBound);
            CMPTLZ_RANGE_NORMALIZE!(range, rangeCode, bufToDec);

            probLen = (probSlot + (CMPTLZ_LEN_CHOICE2!() + posState)).cast();
            c_for!(i = 0; i < CMPTLZ_LOW_LEN_BIT!().cast(); i.suffix_plus_plus(); {
                CMPTLZ_LEN_BIT_DEC!((probLen + decLen), range, rangeCode, rangeBound, decLen, bufToDec);
            });
        } else {
            CMPTLZ_RANGE_UPDATE_1!(probLen, range, rangeCode, rangeBound);
            CMPTLZ_RANGE_NORMALIZE!(range, rangeCode, bufToDec);

            probLen = (probSlot + CMPTLZ_HIGH_LENPROB_OFFSET!()).cast();
            c_for!(i = 0; i < CMPTLZ_HIGH_LEN_BIT!().cast(); i.suffix_plus_plus(); {
                CMPTLZ_LEN_BIT_DEC!((probLen + decLen), range, rangeCode, rangeBound, decLen, bufToDec);
            });
            decLen -= CMPTLZ_HIGH_LEN_CLASS!();
            decLen += (CMPTLZ_LOW_LEN_CLASS!() << 1);
        }
    }

    *pRange = range.cast();
    *pRangeCode = rangeCode.cast();
    *pRangeBound = rangeBound.cast();
    decCtx.buf = bufToDec.cast();

    return decLen.cast();
}
