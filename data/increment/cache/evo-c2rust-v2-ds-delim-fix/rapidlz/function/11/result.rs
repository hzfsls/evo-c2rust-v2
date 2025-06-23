pub fn RapidlzCompWithExternalDict(mut strmCtx: Ptr<RapidlzStreamCtx>, mut src: Ptr<u8>, mut dest: Ptr<u8>, mut srcSize: i32, mut destSize: i32) -> i32 {
    let mut curSrc: Ptr<u8> = src.cast();
    let mut curSrcAnchor: Ptr<u8> = curSrc.cast();
    let mut srcEnd: Ptr<u8> = (curSrc + srcSize).cast();
    let mut curDest: Ptr<u8> = dest.cast();
    let mut destEnd: Ptr<u8> = (curDest + destSize).cast();
    if (srcSize < RAPIDLZ_LAST_LITERAL_LENGTH!()).as_bool() {
        return RapidlzEncLastLiterals(curSrcAnchor.cast(), srcEnd.cast(), curDest.cast(), destEnd.cast(), dest.cast());
    }

    let mut matchStartLimit: Ptr<u8> = (srcEnd - RAPIDLZ_MIN_COMPRESSED_SIZE!() + 1).cast();
    let mut matchEndLimit: Ptr<u8> = (srcEnd - RAPIDLZ_LAST_LITERALS!()).cast();
    let mut startIndex: u32 = strmCtx.currentOffset.cast();
    let mut base: Ptr<u8> = (src.cast::<Ptr<u8>>() - startIndex).cast();

    let mut dict: Ptr<u8> = Default::default();
    let mut dictBase: Ptr<u8> = Default::default();
    let mut dictEnd: Ptr<u8> = Default::default();
    let mut prefixDictStart: Ptr<u8> = Default::default();
    let mut dictSize: u32 = Default::default();
    let mut offset: u32 = 0;

    dict = strmCtx.dict.cast();
    dictSize = strmCtx.dictSize.cast();
    dictBase = (dict + dictSize - strmCtx.currentOffset).cast();
    prefixDictStart = 0;
    dictBase = (dict + dictSize - strmCtx.currentOffset).cast();
    dictEnd = (dict + dictSize).cast();
    strmCtx.dictSize += srcSize;

    let mut prefixDictLimit: u32 = (startIndex - dictSize).cast();
    strmCtx.currentOffset += srcSize.cast::<u32>();

    let mut hashValue: u32 = RapidlzHash4CalcValue(curSrc.cast()).cast();
    RapidlzHash4PutPos(startIndex.cast(), hashValue.cast(), strmCtx.hashTable.cast());
    curSrc += 1;
    let mut forwardHashValue: u32 = RapidlzHash4CalcValue(curSrc.cast()).cast();

    let mut r#match: Ptr<u8> = Default::default();
    let mut token: Ptr<u8> = Default::default();
    let mut acceleration: i32 = strmCtx.acceleration.cast();
    loop {
        let mut forwardPos: Ptr<u8> = curSrc.cast();
        let mut jumpStep: i32 = 1;
        let mut searchMatchNb: i32 = (acceleration << RAPIDLZ_STEP_FORWARD_BASE!()).cast();
        loop {
            hashValue = forwardHashValue.cast();
            let mut current: u32 = (forwardPos - base).cast();
            let mut matchOffset: u32 = RapidlzHash4GetPos(hashValue.cast(), strmCtx.hashTable.cast()).cast();
            curSrc = forwardPos.cast();
            forwardPos += jumpStep;
            jumpStep = (searchMatchNb >> RAPIDLZ_STEP_FORWARD_BASE!()).cast();
            searchMatchNb += 1;

            if RAPIDLZ_UNLIKELY!(forwardPos > matchStartLimit).as_bool() {
                return RapidlzEncLastLiterals(curSrcAnchor.cast(), srcEnd.cast(), curDest.cast(), destEnd.cast(), dest.cast());
            }

            if (matchOffset < startIndex).as_bool() {
                r#match = (dictBase + matchOffset).cast();
                prefixDictStart = dict.cast();
            } else {
                r#match = (base + matchOffset).cast();
                prefixDictStart = src.cast();
            }

            forwardHashValue = RapidlzHash4CalcValue(forwardPos.cast()).cast();
            RapidlzHash4PutPos(current.cast(), hashValue.cast(), strmCtx.hashTable.cast());
            RAPIDLZ_CONTINUE_IF_NOT_A_MATCH!(matchOffset, prefixDictLimit, current);
            if (RAPIDLZ_READ32BIT!(curSrc) == RAPIDLZ_READ32BIT!(r#match)).as_bool() {
                offset = (current - matchOffset).cast();
                break;
            }
        }

        RAPIDLZ_EXPAND_FORWARD!(prefixDictStart, r#match, curSrc, curSrcAnchor);

        token = curDest.cast();
        if !RapidlzStreamEncLiterals(curSrc.cast(), curSrcAnchor.cast(), c_ref!(curDest).cast(), destEnd.cast()).as_bool() {
            return RAPIDLZ_ENC_NOT_OK!();
        }

        _OFFSET_AND_MATCH:

        RapidlzWriteLE16(curDest.cast(), offset.cast());
        curDest += 2;

        let mut matchLen: u32 = Default::default();
        let mut curSrcMatchEnd: Ptr<u8> = Default::default();

        if (prefixDictStart == dict).as_bool() {
            let mut srcLimitOnlyWithDict: Ptr<u8> = (curSrc + (dictEnd - r#match)).cast();
            if (srcLimitOnlyWithDict > matchEndLimit).as_bool() {
                srcLimitOnlyWithDict = matchEndLimit.cast();
            }
            curSrcMatchEnd = RapidlzCompressExpandBackward(srcLimitOnlyWithDict.cast(), (r#match + RAPIDLZ_MIN_MATCH!()).cast(), (curSrc + RAPIDLZ_MIN_MATCH!()).cast()).cast();
            matchLen = (curSrcMatchEnd - curSrc - RAPIDLZ_MIN_MATCH!()).cast::<u32>();
            curSrc = curSrcMatchEnd.cast();

            if (curSrc == srcLimitOnlyWithDict).as_bool() {
                curSrcMatchEnd = RapidlzCompressExpandBackward(matchEndLimit.cast(), src.cast(), srcLimitOnlyWithDict.cast()).cast();
                matchLen += (curSrcMatchEnd - curSrc).cast::<u32>();
                curSrc = curSrcMatchEnd.cast();
            }
        } else {
            curSrcMatchEnd = RapidlzCompressExpandBackward(matchEndLimit.cast(), (r#match + RAPIDLZ_MIN_MATCH!()).cast(), (curSrc + RAPIDLZ_MIN_MATCH!()).cast()).cast();
            matchLen = (curSrcMatchEnd - curSrc - RAPIDLZ_MIN_MATCH!()).cast::<u32>();
            curSrc = curSrcMatchEnd.cast();
        }
        #[cfg(RAPIDLZ_DEBUG)]
        if RAPIDLZ_UNLIKELY!((RAPIDLZ_LIT_AND_MATCH_COPY_END!(curDest, matchLen) > destEnd).as_bool()) {
            return RAPIDLZ_ENC_NOT_OK!();
        }
        curDest += RapidlzStoreMatchLen(matchLen.cast(), token.cast(), curDest.cast()).cast();
        curSrcAnchor = curSrc.cast();
        if (curSrc >= matchStartLimit).as_bool() {
            break;
        }
        let mut hv2: u32 = RapidlzHash4CalcValue((curSrc - 2).cast()).cast();
        let mut index: u32 = (curSrc - 2 - base).cast();
        RapidlzHash4PutPos(index.cast(), hv2.cast(), strmCtx.hashTable.cast());

        hashValue = RapidlzHash4CalcValue(curSrc.cast()).cast();
        let mut current: u32 = (curSrc - base).cast();
        let mut matchOffset: u32 = RapidlzHash4GetPos(hashValue.cast(), strmCtx.hashTable.cast()).cast();
        if (matchOffset < startIndex).as_bool() {
            r#match = (dictBase + matchOffset).cast();
            prefixDictStart = dict.cast();
        } else {
            r#match = (base + matchOffset).cast();
            prefixDictStart = src.cast();
        }

        RapidlzHash4PutPos(current.cast(), hashValue.cast(), strmCtx.hashTable.cast());

        if (matchOffset >= prefixDictLimit).as_bool() && (matchOffset + RAPIDLZ_MAX_OFFSET!() >= current).as_bool() {
            if (RAPIDLZ_READ32BIT!(curSrc) == RAPIDLZ_READ32BIT!(r#match)).as_bool() {
                token = curDest.cast();
                curDest += 1;
                *token = 0;
                offset = (current - matchOffset).cast();
                goto _OFFSET_AND_MATCH;
            }
        }
        forwardHashValue = RapidlzHash4CalcValue(curSrc.prefix_plus_plus().cast()).cast();
    }

    return RapidlzEncLastLiterals(curSrcAnchor.cast(), srcEnd.cast(), curDest.cast(), destEnd.cast(), dest.cast());
}
