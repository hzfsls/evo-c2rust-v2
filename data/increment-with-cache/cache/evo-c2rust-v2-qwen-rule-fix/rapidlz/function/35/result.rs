pub fn RapidlzCompressExpandBackward(mut matchLimit: Ptr<u8>, mut matchPtr: Ptr<u8>, mut srcCurr: Ptr<u8>) -> Ptr<u8> {
    let mut xorVal: u64;
    let mut loopEnd: Ptr<u8> = matchLimit - 7;
    let mut srcCurrMatchEnd: Ptr<u8> = srcCurr;
    let mut matchBegin: Ptr<u8> = matchPtr;

    while (srcCurrMatchEnd < loopEnd) {
        xorVal = RAPIDLZ_READ64BIT!(matchBegin) ^ RAPIDLZ_READ64BIT!(srcCurrMatchEnd);
        if (RAPIDLZ_UNLIKELY!(xorVal == 0)) {
            srcCurrMatchEnd += c_sizeof!(u64);
            matchBegin += c_sizeof!(u64);
            continue;
        }
        srcCurrMatchEnd += if RapidlzIsLE().as_bool() {
            (RapidlzCountTailZero64(xorVal) >> 3)
        } else {
            (RapidlzCountLeadZero64(xorVal) >> 3)
        };
        return srcCurrMatchEnd;
    }

    if ((srcCurrMatchEnd + 3) < matchLimit) && (RAPIDLZ_READ32BIT!(srcCurrMatchEnd) == RAPIDLZ_READ32BIT!(matchBegin)) {
        srcCurrMatchEnd += c_sizeof!(u32);
        matchBegin += c_sizeof!(u32);
    }

    if ((srcCurrMatchEnd + 1) < matchLimit) && (RAPIDLZ_READ16BIT!(srcCurrMatchEnd) == RAPIDLZ_READ16BIT!(matchBegin)) {
        srcCurrMatchEnd += c_sizeof!(u16);
        matchBegin += c_sizeof!(u16);
    }

    if (srcCurrMatchEnd < matchLimit) && (srcCurrMatchEnd[0] == matchBegin[0]) {
        srcCurrMatchEnd += 1;
    }
    return srcCurrMatchEnd;
}