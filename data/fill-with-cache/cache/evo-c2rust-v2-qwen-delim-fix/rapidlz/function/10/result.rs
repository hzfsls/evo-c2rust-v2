pub fn RapidlzCompressExpandBackward(mut matchLimit: Ptr<u8>, mut matchPtr: Ptr<u8>, mut srcCurr: Ptr<u8>) -> Ptr<u8> {
    let mut xorVal: u64;
    let mut loopEnd: Ptr<u8> = matchLimit - 7;
    let mut srcCurrMatchEnd: Ptr<u8> = srcCurr.cast();
    let mut matchBegin: Ptr<u8> = matchPtr.cast();
    while (srcCurrMatchEnd < loopEnd).as_bool() {
        xorVal = RAPIDLZ_READ64BIT!(matchBegin).cast() ^ RAPIDLZ_READ64BIT!(srcCurrMatchEnd).cast();
        if (RAPIDLZ_UNLIKELY!(xorVal == 0).as_bool()) {
            srcCurrMatchEnd += c_sizeof!(u64).cast();
            matchBegin += c_sizeof!(u64).cast();
            continue;
        }
        srcCurrMatchEnd += if RapidlzIsLE().as_bool() {
            (RapidlzCountTailZero64(xorVal) >> 3).cast()
        } else {
            (RapidlzCountLeadZero64(xorVal) >> 3).cast()
        };
        return srcCurrMatchEnd.cast();
    }
    if ((srcCurrMatchEnd + 3) < matchLimit).as_bool() && (RAPIDLZ_READ32BIT!(srcCurrMatchEnd) == RAPIDLZ_READ32BIT!(matchBegin)).as_bool() {
        srcCurrMatchEnd += c_sizeof!(u32).cast();
        matchBegin += c_sizeof!(u32).cast();
    }
    if ((srcCurrMatchEnd + 1) < matchLimit).as_bool() && (RAPIDLZ_READ16BIT!(srcCurrMatchEnd) == RAPIDLZ_READ16BIT!(matchBegin)).as_bool() {
        srcCurrMatchEnd += c_sizeof!(u16).cast();
        matchBegin += c_sizeof!(u16).cast();
    }
    if (srcCurrMatchEnd < matchLimit).as_bool() && (srcCurrMatchEnd[0] == matchBegin[0]).as_bool() {
        srcCurrMatchEnd += 1;
    }
    return srcCurrMatchEnd.cast();
}