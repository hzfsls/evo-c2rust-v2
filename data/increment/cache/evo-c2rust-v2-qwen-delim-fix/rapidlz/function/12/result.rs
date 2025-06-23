pub fn RapidlzCompWithPrefixDict(mut strmCtx: Ptr<RapidlzStreamCtx>, mut src: Ptr<u8>, mut dest: Ptr<u8>, mut srcSize: i32, mut destSize: i32) -> i32 {
    let mut curSrc: Ptr<u8> = src.cast();
    let mut curSrcAnchor: Ptr<u8> = curSrc.cast();
    let mut srcEnd: Ptr<u8> = (curSrc + srcSize).cast();
    let mut curDest: Ptr<u8> = dest.cast();
    let mut destEnd: Ptr<u8> = (curDest + destSize).cast();
    if (srcSize < RAPIDLZ_LAST_LITERAL_LENGTH!()).as_bool() {
        return RapidlzEncLastLiterals(curSrcAnchor.cast(), srcEnd.cast(), curDest.cast(), destEnd.cast(), dest.cast()).cast();
    }
    let mut matchStartLimit: Ptr<u8> = (srcEnd - RAPIDLZ_MIN_COMPRESSED_SIZE!() + 1).cast();
    let mut matchEndLimit: Ptr<u8> = (srcEnd - RAPIDLZ_LAST_LITERALS!()).cast();
    let mut startIndex: u32 = strmCtx.currentOffset.cast();
    let mut base: Ptr<u8> = (curSrc - startIndex).cast();
    let mut prefixDictStart: Ptr<u8> = (curSrc - strmCtx.dictSize.cast()).cast();
    let mut dictSize: u32 = strmCtx.dictSize.cast();
    strmCtx.dictSize += srcSize.cast();
    let mut prefixDictLimit: u32 = startIndex - dictSize;
    strmCtx.currentOffset += srcSize.cast();
    let mut hashValue: u32 = RapidlzHash4CalcValue(curSrc.cast()).cast();
    RapidlzHash4PutPos(startIndex, hashValue, strmCtx.hashTable.cast());
    curSrc += 1;
    let mut forwardHashValue: u32 = RapidlzHash4CalcValue(curSrc.cast()).cast();
    let mut match: Ptr<u8> = Default::default();
    let mut token: Ptr<u8> = Default::default();
    let mut acceleration: i32 = strmCtx.acceleration.cast();
    loop {
        let mut forwardPos: Ptr<u8> = curSrc.cast();
        let mut step: i32 = 1;
        let mut searchMatchNb: i32 = acceleration << RAPIDLZ_STEP_FORWARD_BASE!();
        loop {
            hashValue = forwardHashValue;
            let mut current: u32 = (forwardPos - base).cast();
            let mut matchOffset: u32 = RapidlzHash4GetPos(hashValue, strmCtx.hashTable.cast()).cast();
            curSrc = forwardPos.cast();
            forwardPos += step;
            step = (searchMatchNb >> RAPIDLZ_STEP_FORWARD_BASE!()).cast();
            searchMatchNb += 1;
            if (forwardPos > matchStartLimit).as_bool() {
                return RapidlzEncLastLiterals(curSrcAnchor.cast(), srcEnd.cast(), curDest.cast(), destEnd.cast(), dest.cast()).cast();
            }
            match = (base + matchOffset).cast();
            forwardHashValue = RapidlzHash4CalcValue(forwardPos.cast()).cast();
            RapidlzHash4PutPos(current, hashValue, strmCtx.hashTable.cast());
            if (matchOffset < prefixDictLimit).as_bool() {
                continue;
            }
            if ((matchOffset + RAPIDLZ_MAX_OFFSET!()) < current).as_bool() {
                continue;
            }
            if (RAPIDLZ_READ32BIT!(curSrc) == RAPIDLZ_READ32BIT!(match)).as_bool() {
                break;
            }
        }
        RAPIDLZ_EXPAND_FORWARD!(prefixDictStart, match, curSrc, curSrcAnchor);
        token = curDest.cast();
        if !RAPIDLZ_RETURN_IF_NOT_TRUE!(RapidlzStreamEncLiterals(curSrc.cast(), curSrcAnchor.cast(), c_ref!(curDest).cast(), destEnd.cast()), RAPIDLZ_ENC_NOT_OK!()).as_bool() {
            return RAPIDLZ_ENC_NOT_OK!();
        }
        _OFFSET_AND_MATCH!():
        RapidlzWriteLE16!(curDest, curSrc - match).cast();
        curDest += 2;
        let mut matchLen: u32 = Default::default();
        let mut curSrcMatchEnd: Ptr<u8> = Default::default();
        curSrcMatchEnd = RapidlzCompressExpandBackward!(matchEndLimit, match + RAPIDLZ_MIN_MATCH!(), curSrc + RAPIDLZ_MIN_MATCH!()).cast();
        matchLen = (curSrcMatchEnd - curSrc - RAPIDLZ_MIN_MATCH!).cast();
        curSrc = curSrcMatchEnd.cast();
        curDest += RapidlzStoreMatchLen!(matchLen, token, curDest).cast();
        curSrcAnchor = curSrc.cast();
        if (curSrc >= matchStartLimit).as_bool() {
            break;
        }
        let mut hv: u32 = RapidlzHash4CalcValue!(curSrc - 2).cast();
        let mut index: u32 = (curSrc - 2 - base).cast();
        RapidlzHash4PutPos!(index, hv, strmCtx.hashTable.cast());
        hashValue = RapidlzHash4CalcValue!(curSrc).cast();
        let mut current: u32 = (curSrc - base).cast();
        let mut matchOffset: u32 = RapidlzHash4GetPos!(hashValue, strmCtx.hashTable.cast()).cast();
        match = (base + matchOffset).cast();
        RapidlzHash4PutPos!(current, hashValue, strmCtx.hashTable.cast());
        if (matchOffset >= prefixDictLimit).as_bool() && ((matchOffset + RAPIDLZ_MAX_OFFSET!()) >= current).as_bool() {
            if (RAPIDLZ_READ32BIT!(curSrc) == RAPIDLZ_READ32BIT!(match)).as_bool() {
                token = curDest;
                curDest += 1;
                *token = 0;
                goto _OFFSET_AND_MATCH;
            }
        }
        forwardHashValue = RapidlzHash4CalcValue!(curSrc + 1).cast();
    }
    return RapidlzEncLastLiterals!(curSrcAnchor, srcEnd, curDest, destEnd, dest).cast();
}