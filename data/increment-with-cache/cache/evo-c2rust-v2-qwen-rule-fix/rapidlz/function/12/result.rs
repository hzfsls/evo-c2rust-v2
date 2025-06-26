pub fn RapidlzCompWithPrefixDict(mut strmCtx: Ptr<RapidlzStreamCtx>, mut src: Ptr<u8>, mut dest: Ptr<u8>, mut srcSize: i32, mut destSize: i32) -> i32 {
    let mut curSrc: Ptr<u8> = src;
    let mut curSrcAnchor: Ptr<u8> = curSrc;
    let mut srcEnd: Ptr<u8> = (curSrc + srcSize);
    let mut curDest: Ptr<u8> = dest;
    let mut destEnd: Ptr<u8> = (curDest + destSize);
    if (srcSize < RAPIDLZ_LAST_LITERAL_LENGTH!()) {
        return RapidlzEncLastLiterals(curSrcAnchor, srcEnd, curDest, destEnd, dest.cast());
    }
    let mut matchStartLimit: Ptr<u8> = (srcEnd - RAPIDLZ_MIN_COMPRESSED_SIZE!() + 1);
    let mut matchEndLimit: Ptr<u8> = (srcEnd - RAPIDLZ_LAST_LITERALS!());
    let mut startIndex: u32 = strmCtx.currentOffset;
    let mut base: Ptr<u8> = (curSrc - startIndex);
    let mut prefixDictStart: Ptr<u8> = (curSrc - strmCtx.dictSize);
    let mut dictSize: u32 = strmCtx.dictSize;
    strmCtx.dictSize += srcSize.cast();
    let mut prefixDictLimit: u32 = startIndex - dictSize;
    strmCtx.currentOffset += srcSize.cast();
    let mut hashValue: u32 = RapidlzHash4CalcValue(curSrc);
    RapidlzHash4PutPos(startIndex, hashValue, strmCtx.hashTable.cast());
    curSrc += 1;
    let mut forwardHashValue: u32 = RapidlzHash4CalcValue(curSrc);
    let mut match: Ptr<u8> = Default::default();
    let mut token: Ptr<u8> = Default::default();
    let mut acceleration: i32 = strmCtx.acceleration;
    loop {
        let mut forwardPos: Ptr<u8> = curSrc;
        let mut step: i32 = 1;
        let mut searchMatchNb: i32 = acceleration << RAPIDLZ_STEP_FORWARD_BASE!();
        loop {
            hashValue = forwardHashValue;
            let mut current: u32 = (forwardPos - base).cast();
            let mut matchOffset: u32 = RapidlzHash4GetPos(hashValue, strmCtx.hashTable.cast());
            curSrc = forwardPos;
            forwardPos += step;
            step = (searchMatchNb >> RAPIDLZ_STEP_FORWARD_BASE!());
            searchMatchNb += 1;
            if (forwardPos > matchStartLimit) {
                return RapidlzEncLastLiterals(curSrcAnchor, srcEnd, curDest, destEnd, dest.cast());
            }
            match = (base + matchOffset);
            forwardHashValue = RapidlzHash4CalcValue(forwardPos);
            RapidlzHash4PutPos(current, hashValue, strmCtx.hashTable);
            if (matchOffset < prefixDictLimit) {
                continue;
            }
            if ((matchOffset + RAPIDLZ_MAX_OFFSET!()) < current) {
                continue;
            }
            if (RAPIDLZ_READ32BIT!(curSrc) == RAPIDLZ_READ32BIT!(match)) {
                break;
            }
        }
        RAPIDLZ_EXPAND_FORWARD!(prefixDictStart, match, curSrc, curSrcAnchor);
        token = curDest;
        if !RAPIDLZ_RETURN_IF_NOT_TRUE!(RapidlzStreamEncLiterals(curSrc, curSrcAnchor, c_ref!(curDest), destEnd), RAPIDLZ_ENC_NOT_OK!()).as_bool() {
            return RAPIDLZ_ENC_NOT_OK!();
        }
        _OFFSET_AND_MATCH!():
        RapidlzWriteLE16!(curDest, curSrc - match);
        curDest += 2;
        let mut matchLen: u32 = Default::default();
        let mut curSrcMatchEnd: Ptr<u8> = Default::default();
        curSrcMatchEnd = RapidlzCompressExpandBackward!(matchEndLimit, match + RAPIDLZ_MIN_MATCH!(), curSrc + RAPIDLZ_MIN_MATCH!());
        matchLen = (curSrcMatchEnd - curSrc - RAPIDLZ_MIN_MATCH!);
        curSrc = curSrcMatchEnd;
        curDest += RapidlzStoreMatchLen!(matchLen, token, curDest);
        curSrcAnchor = curSrc;
        if (curSrc >= matchStartLimit) {
            break;
        }
        let mut hv: u32 = RapidlzHash4CalcValue!(curSrc - 2);
        let mut index: u32 = (curSrc - 2 - base).cast();
        RapidlzHash4PutPos!(index, hv, strmCtx.hashTable);
        hashValue = RapidlzHash4CalcValue!(curSrc);
        let mut current: u32 = (curSrc - base).cast();
        let mut matchOffset: u32 = RapidlzHash4GetPos!(hashValue, strmCtx.hashTable);
        match = (base + matchOffset);
        RapidlzHash4PutPos!(current, hashValue, strmCtx.hashTable);
        if (matchOffset >= prefixDictLimit) && ((matchOffset + RAPIDLZ_MAX_OFFSET!()) >= current) {
            if (RAPIDLZ_READ32BIT!(curSrc) == RAPIDLZ_READ32BIT!(match)) {
                token = curDest;
                curDest += 1;
                *token = 0;
                goto _OFFSET_AND_MATCH;
            }
        }
        forwardHashValue = RapidlzHash4CalcValue!(curSrc + 1);
    }
    return RapidlzEncLastLiterals!(curSrcAnchor, srcEnd, curDest, destEnd, dest);
}