pub fn RapidlzCompWithExternalDict(mut strmCtx: Ptr<RapidlzStreamCtx>, mut src: Ptr<u8>, mut dest: Ptr<u8>, mut srcSize: i32, mut destSize: i32) -> i32 {
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
    let mut base: Ptr<u8> = (src.cast::<Ptr<u8>>() - startIndex);

    let mut dict: Ptr<u8> = Default::default();
    let mut dictBase: Ptr<u8> = Default::default();
    let mut dictEnd: Ptr<u8> = Default::default();
    let mut prefixDictStart: Ptr<u8> = Default::default();
    let mut dictSize: u32 = Default::default();
    let mut offset: u32 = 0;

    dict = strmCtx.dict;
    dictSize = strmCtx.dictSize;
    dictBase = (dict + dictSize - strmCtx.currentOffset);
    prefixDictStart = 0;
    dictBase = (dict + dictSize - strmCtx.currentOffset);
    dictEnd = (dict + dictSize);
    strmCtx.dictSize += srcSize;

    let mut prefixDictLimit: u32 = (startIndex - dictSize);
    strmCtx.currentOffset += srcSize.cast::<u32>();

    let mut hashValue: u32 = RapidlzHash4CalcValue(curSrc);
    RapidlzHash4PutPos(startIndex, hashValue, strmCtx.hashTable.cast());
    curSrc += 1;
    let mut forwardHashValue: u32 = RapidlzHash4CalcValue(curSrc);

    let mut r#match: Ptr<u8> = Default::default();
    let mut token: Ptr<u8> = Default::default();
    let mut acceleration: i32 = strmCtx.acceleration;
    loop {
        let mut forwardPos: Ptr<u8> = curSrc;
        let mut jumpStep: i32 = 1;
        let mut searchMatchNb: i32 = (acceleration << RAPIDLZ_STEP_FORWARD_BASE!());
        loop {
            hashValue = forwardHashValue;
            let mut current: u32 = (forwardPos - base).cast();
            let mut matchOffset: u32 = RapidlzHash4GetPos(hashValue, strmCtx.hashTable.cast());
            curSrc = forwardPos;
            forwardPos += jumpStep;
            jumpStep = (searchMatchNb >> RAPIDLZ_STEP_FORWARD_BASE!());
            searchMatchNb += 1;

            if RAPIDLZ_UNLIKELY!(forwardPos > matchStartLimit) {
                return RapidlzEncLastLiterals(curSrcAnchor, srcEnd, curDest, destEnd, dest);
            }

            if (matchOffset < startIndex) {
                r#match = (dictBase + matchOffset);
                prefixDictStart = dict;
            } else {
                r#match = (base + matchOffset);
                prefixDictStart = src;
            }

            forwardHashValue = RapidlzHash4CalcValue(forwardPos);
            RapidlzHash4PutPos(current, hashValue, strmCtx.hashTable.cast());
            RAPIDLZ_CONTINUE_IF_NOT_A_MATCH!(matchOffset, prefixDictLimit, current);
            if (RAPIDLZ_READ32BIT!(curSrc) == RAPIDLZ_READ32BIT!(r#match)) {
                offset = (current - matchOffset);
                break;
            }
        }

        RAPIDLZ_EXPAND_FORWARD!(prefixDictStart, r#match, curSrc, curSrcAnchor);

        token = curDest;
        if !RapidlzStreamEncLiterals(curSrc, curSrcAnchor, c_ref!(curDest), destEnd) {
            return RAPIDLZ_ENC_NOT_OK!();
        }

        _OFFSET_AND_MATCH:

        RapidlzWriteLE16(curDest, offset);
        curDest += 2;

        let mut matchLen: u32 = Default::default();
        let mut curSrcMatchEnd: Ptr<u8> = Default::default();

        if (prefixDictStart == dict) {
            let mut srcLimitOnlyWithDict: Ptr<u8> = (curSrc + (dictEnd - r#match));
            if (srcLimitOnlyWithDict > matchEndLimit) {
                srcLimitOnlyWithDict = matchEndLimit;
            }
            curSrcMatchEnd = RapidlzCompressExpandBackward(srcLimitOnlyWithDict, (r#match + RAPIDLZ_MIN_MATCH!()), (curSrc + RAPIDLZ_MIN_MATCH!()));
            matchLen = (curSrcMatchEnd - curSrc - RAPIDLZ_MIN_MATCH!()).cast::<u32>();
            curSrc = curSrcMatchEnd;

            if (curSrc == srcLimitOnlyWithDict) {
                curSrcMatchEnd = RapidlzCompressExpandBackward(matchEndLimit, src, srcLimitOnlyWithDict);
                matchLen += (curSrcMatchEnd - curSrc).cast::<u32>();
                curSrc = curSrcMatchEnd;
            }
        } else {
            curSrcMatchEnd = RapidlzCompressExpandBackward(matchEndLimit, (r#match + RAPIDLZ_MIN_MATCH!()), (curSrc + RAPIDLZ_MIN_MATCH!()));
            matchLen = (curSrcMatchEnd - curSrc - RAPIDLZ_MIN_MATCH!()).cast::<u32>();
            curSrc = curSrcMatchEnd;
        }
        #[cfg(RAPIDLZ_DEBUG)]
        if RAPIDLZ_UNLIKELY!((RAPIDLZ_LIT_AND_MATCH_COPY_END!(curDest, matchLen) > destEnd)) {
            return RAPIDLZ_ENC_NOT_OK!();
        }
        curDest += RapidlzStoreMatchLen(matchLen, token, curDest);
        curSrcAnchor = curSrc;
        if (curSrc >= matchStartLimit) {
            break;
        }
        let mut hv2: u32 = RapidlzHash4CalcValue((curSrc - 2));
        let mut index: u32 = (curSrc - 2 - base);
        RapidlzHash4PutPos(index, hv2, strmCtx.hashTable);

        hashValue = RapidlzHash4CalcValue(curSrc);
        let mut current: u32 = (curSrc - base);
        let mut matchOffset: u32 = RapidlzHash4GetPos(hashValue, strmCtx.hashTable);
        if (matchOffset < startIndex) {
            r#match = (dictBase + matchOffset);
            prefixDictStart = dict;
        } else {
            r#match = (base + matchOffset);
            prefixDictStart = src;
        }

        RapidlzHash4PutPos(current, hashValue, strmCtx.hashTable);

        if (matchOffset >= prefixDictLimit) && (matchOffset + RAPIDLZ_MAX_OFFSET!() >= current) {
            if (RAPIDLZ_READ32BIT!(curSrc) == RAPIDLZ_READ32BIT!(r#match)) {
                token = curDest;
                curDest += 1;
                *token = 0;
                offset = (current - matchOffset);
                goto _OFFSET_AND_MATCH;
            }
        }
        forwardHashValue = RapidlzHash4CalcValue(curSrc.prefix_plus_plus());
    }

    return RapidlzEncLastLiterals(curSrcAnchor, srcEnd, curDest, destEnd, dest);
}
