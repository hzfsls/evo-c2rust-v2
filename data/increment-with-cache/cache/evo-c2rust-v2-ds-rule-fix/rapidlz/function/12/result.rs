pub fn RapidlzCompWithPrefixDict(mut strmCtx: Ptr<RapidlzStreamCtx>, mut src: Ptr<u8>, mut dest: Ptr<u8>, mut srcSize: i32, mut destSize: i32) -> i32 {
    let mut curSrc: Ptr<u8> = src;
    let mut curSrcAnchor: Ptr<u8> = curSrc;
    let mut srcEnd: Ptr<u8> = (curSrc + srcSize);
    let mut curDest: Ptr<u8> = dest;
    let mut destEnd: Ptr<u8> = (curDest + destSize);
    if (srcSize < RAPIDLZ_LAST_LITERAL_LENGTH!()) {
        return RapidlzEncLastLiterals(curSrcAnchor, srcEnd, curDest, destEnd, dest);
    }
    let mut matchStartLimit: Ptr<u8> = (srcEnd - RAPIDLZ_MIN_COMPRESSED_SIZE!() + 1);
    let mut matchEndLimit: Ptr<u8> = (srcEnd - RAPIDLZ_LAST_LITERALS!());
    let mut startIndex: u32 = strmCtx.currentOffset;
    let mut base: Ptr<u8> = (src - startIndex);

    let mut prefixDictStart: Ptr<u8>;
    let mut dictSize: u32;
    dictSize = strmCtx.dictSize;
    prefixDictStart = (src - dictSize);
    strmCtx.dictSize += srcSize;
    let mut prefixDictLimit: u32 = (startIndex - dictSize);
    strmCtx.currentOffset += srcSize.cast::<u32>();

    let mut hashValue: u32 = RapidlzHash4CalcValue(curSrc);
    RapidlzHash4PutPos(startIndex, hashValue, strmCtx.hashTable.cast());
    curSrc += 1;
    let mut forwardHashValue: u32 = RapidlzHash4CalcValue(curSrc);

    let mut r#match: Ptr<u8>;
    let mut token: Ptr<u8>;
    let mut acceleration: i32 = strmCtx.acceleration;
    loop {
        let mut forwardPos: Ptr<u8> = curSrc;
        let mut step: i32 = 1;
        let mut searchMatchNb: i32 = (acceleration << RAPIDLZ_STEP_FORWARD_BASE!());
        loop {
            hashValue = forwardHashValue;
            let mut current: u32 = (forwardPos - base).cast();
            let mut matchOffset: u32 = RapidlzHash4GetPos(hashValue, strmCtx.hashTable.cast());
            curSrc = forwardPos;
            forwardPos += step;
            step = (searchMatchNb >> RAPIDLZ_STEP_FORWARD_BASE!());
            searchMatchNb += 1;

            if RAPIDLZ_UNLIKELY!(forwardPos > matchStartLimit) {
                return RapidlzEncLastLiterals(curSrcAnchor, srcEnd, curDest, destEnd, dest);
            }

            r#match = (base + matchOffset);
            forwardHashValue = RapidlzHash4CalcValue(forwardPos);
            RapidlzHash4PutPos(current, hashValue, strmCtx.hashTable.cast());

            if (matchOffset < prefixDictLimit) {
                continue;
            }
            if ((matchOffset + RAPIDLZ_MAX_OFFSET!()) < current) {
                continue;
            }
            if (RAPIDLZ_READ32BIT!(curSrc) == RAPIDLZ_READ32BIT!(r#match)) {
                break;
            }
        }

        RAPIDLZ_EXPAND_FORWARD!(prefixDictStart, r#match, curSrc, curSrcAnchor);

        token = curDest;
        if !RapidlzStreamEncLiterals(curSrc, curSrcAnchor, c_ref!(curDest), destEnd) {
            return RAPIDLZ_ENC_NOT_OK!();
        }

        _OFFSET_AND_MATCH:

        RapidlzWriteLE16(curDest, (curSrc - r#match));
        curDest += 2;

        let mut matchLen: u32;
        let mut curSrcMatchEnd: Ptr<u8>;

        curSrcMatchEnd = RapidlzCompressExpandBackward(matchEndLimit, (r#match + RAPIDLZ_MIN_MATCH!()), (curSrc + RAPIDLZ_MIN_MATCH!())).cast::<Ptr<u8>>();
        matchLen = (curSrcMatchEnd - curSrc - RAPIDLZ_MIN_MATCH!());
        curSrc = curSrcMatchEnd;
        if RAPIDLZ_UNLIKELY!(RAPIDLZ_LIT_AND_MATCH_COPY_END!(curDest, matchLen) > destEnd) {
            return RAPIDLZ_ENC_NOT_OK!();
        }
        curDest += RapidlzStoreMatchLen(matchLen, token, curDest);

        curSrcAnchor = curSrc;
        if (curSrc >= matchStartLimit) {
            break;
        }
        let mut hv: u32 = RapidlzHash4CalcValue((curSrc - 2));
        let mut index: u32 = (curSrc - 2 - base);
        RapidlzHash4PutPos(index, hv, strmCtx.hashTable);

        hashValue = RapidlzHash4CalcValue(curSrc);
        let mut current: u32 = (curSrc - base);
        let mut matchOffset: u32 = RapidlzHash4GetPos(hashValue, strmCtx.hashTable);

        r#match = (base + matchOffset);

        RapidlzHash4PutPos(current, hashValue, strmCtx.hashTable);
        if (matchOffset >= prefixDictLimit) && ((matchOffset + RAPIDLZ_MAX_OFFSET!()) >= current) {
            if (RAPIDLZ_READ32BIT!(curSrc) == RAPIDLZ_READ32BIT!(r#match)) {
                token = curDest;
                curDest += 1;
                *token = 0;
                goto _OFFSET_AND_MATCH;
            }
        }
        forwardHashValue = RapidlzHash4CalcValue(curSrc.prefix_plus_plus());
    }

    return RapidlzEncLastLiterals(curSrcAnchor, srcEnd, curDest, destEnd, dest);
}
