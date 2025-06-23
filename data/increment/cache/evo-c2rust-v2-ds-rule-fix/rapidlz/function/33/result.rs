pub fn RapidlzCompressExpandBackward(mut matchLimit: Ptr<u8>, mut matchPtr: Ptr<u8>, mut srcCurr: Ptr<u8>) -> Ptr<u8> {
    let mut xorVal: u64;
    let mut loopEnd: Ptr<u8> = (matchLimit - 7).cast();
    let mut srcCurrMatchEnd: Ptr<u8> = srcCurr.cast();
    let mut matchBegin: Ptr<u8> = matchPtr.cast();
    while (srcCurrMatchEnd < loopEnd).as_bool() {
        xorVal = RAPIDLZ_READ64BIT!(matchBegin) ^ RAPIDLZ_READ64BIT!(srcCurrMatchEnd);
        if RAPIDLZ_UNLIKELY!(xorVal == 0).as_bool() {
            srcCurrMatchEnd += c_sizeof!(u64);
            matchBegin += c_sizeof!(u64);
            continue;
        }
        srcCurrMatchEnd += if RapidlzIsLE().as_bool() {
            RapidlzCountTailZero64(xorVal) >> 3
        } else {
            RapidlzCountLeadZero64(xorVal) >> 3
        };
        return srcCurrMatchEnd.cast();
    }
    if ((srcCurrMatchEnd + 3) < matchLimit).as_bool() && (RAPIDLZ_READ32BIT!(srcCurrMatchEnd) == RAPIDLZ_READ32BIT!(matchBegin)).as_bool() {
        srcCurrMatchEnd += c_sizeof!(u32);
        matchBegin += c_sizeof!(u32);
    }
    if ((srcCurrMatchEnd + 1) < matchLimit).as_bool() && (RAPIDLZ_READ16BIT!(srcCurrMatchEnd) == RAPIDLZ_READ16BIT!(matchBegin)).as_bool() {
        srcCurrMatchEnd += c_sizeof!(u16);
        matchBegin += c_sizeof!(u16);
    }
    if (srcCurrMatchEnd < matchLimit).as_bool() && (srcCurrMatchEnd[0] == matchBegin[0]).as_bool() {
        srcCurrMatchEnd += 1;
    }
    return srcCurrMatchEnd.cast();
}
