pub fn RapidlzDecWithPrefixDict(mut src: Ptr<u8>, mut dest: Ptr<u8>, mut srcSize: i32, mut outBufferSize: i32, mut dictStart: Ptr<u8>, mut dictSize: i32) -> i32 {
    c_ref!(dictStart).cast::<Void>();
    c_ref!(dictSize).cast::<Void>();
    c_if!(RAPIDLZ_DEBUG!(), {
        dictSize = if dictSize > RAPIDLZ_MAX_DICT_SIZE!() { RAPIDLZ_MAX_DICT_SIZE!() } else { dictSize };
        let mut prefixDictStart: Ptr<u8> = (dest.cast::<Ptr<u8>>() - dictSize);
    });
    let mut curSrc: Ptr<u8> = src.cast::<Ptr<u8>>();
    let mut srcEnd: Ptr<u8> = (curSrc + srcSize);
    let mut curDest: Ptr<u8> = dest.cast::<Ptr<u8>>();
    let mut destEnd: Ptr<u8> = (curDest + outBufferSize);

    let mut srcEndFast: Ptr<u8> = (srcEnd - RAPIDLZ_COPY_PROTECT_SIZE!());
    let mut destEndFast: Ptr<u8> = (destEnd - RAPIDLZ_COPY_PROTECT_SIZE!());

    let mut token: u32 = Default::default();
    let mut len: u32 = Default::default();
    let mut offset: u16 = Default::default();
    let mut matchSrc: Ptr<u8> = Default::default();
    let mut tmp: u32 = Default::default();
    let mut leftSrcSize: usize = Default::default();
    loop {
        token = *curSrc;
        curSrc += 1;

        len = (token >> 4);
        if RAPIDLZ_LIKELY!(len < RAPIDLZ_MAX_4BIT_VALUE!()) {
            if RAPIDLZ_LIKELY!((curSrc + len <= srcEndFast) && (curDest + len <= destEndFast)) {
                RapidlzCopy16Byte(curDest, curSrc);
                RAPIDLZ_POSITION_UPDATE!(curSrc, curDest, len);
            } else {
                leftSrcSize = (srcEnd - curSrc);
                RAPIDLZ_SAFE_COPY_TILL_END!(curSrc, leftSrcSize, curDest, destEnd, len);
            }
        } else {
            RAPIDLZ_READ_OPTIONAL_LENGTH!(len, curSrc, srcEnd, tmp);
            if RAPIDLZ_LIKELY!((curSrc + len <= srcEndFast) && (curDest + len <= destEndFast)) {
                RapidlzWildCopy16(curSrc, curDest, (curDest + len));
                RAPIDLZ_POSITION_UPDATE!(curSrc, curDest, len);
            } else {
                leftSrcSize = (srcEnd - curSrc);
                RAPIDLZ_SAFE_COPY_TILL_END!(curSrc, leftSrcSize, curDest, destEnd, len);
            }
        }

        offset = RapidlzReadLE16Bit(curSrc);
        curSrc += 2;
        matchSrc = (curDest - offset);
        c_if!(RAPIDLZ_DEBUG!(), {
            RAPIDLZ_RETURN_IF_NOT_TRUE!(RAPIDLZ_UNLIKELY!(matchSrc < prefixDictStart) == false, RAPIDLZ_DEC_NOT_OK!());
        });

        len = (token & RAPIDLZ_MAX_4BIT_VALUE!());
        if len < RAPIDLZ_MAX_4BIT_VALUE!() {
            len += 4;
        } else {
            RAPIDLZ_READ_OPTIONAL_LENGTH!(len, curSrc, srcEnd, tmp);
            len += 4;
        }
        c_if!(RAPIDLZ_DEBUG!(), {
            RAPIDLZ_RETURN_IF_NOT_TRUE!((curDest + len) <= (destEnd - RAPIDLZ_LAST_LITERALS!()), RAPIDLZ_DEC_NOT_OK!());
        });

        if RAPIDLZ_LIKELY!((curDest + len) <= (destEndFast - RAPIDLZ_COPY_PROTECT_SIZE!() + RAPIDLZ_LAST_LITERALS!())) {
            RapidlzCopyMatchFast(curDest, matchSrc, offset, len);
            curDest += len;
        } else {
            if RAPIDLZ_LIKELY!(len < 1024) {
                RAPIDLZ_FAST_SAFE_COPY_BY_BYTES!(curDest, matchSrc, len);
            } else {
                RapidlzSafeCopyMatchFast(curDest, matchSrc, destEnd, offset, len);
                curDest += len;
            }
        }
    }

    return (curDest.cast::<Void>() - dest.cast::<Void>()).cast::<i32>();
}