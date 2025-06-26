pub fn RapidlzDecWithPrefixDict(mut src: Ptr<u8>, mut dest: Ptr<u8>, mut srcSize: i32, mut outBufferSize: i32, mut dictStart: Ptr<u8>, mut dictSize: i32) -> i32 {
    dictStart.cast::<Void>();
    dictSize.cast::<Void>();
    dictSize = if dictSize > RAPIDLZ_MAX_DICT_SIZE!() { RAPIDLZ_MAX_DICT_SIZE!() } else { dictSize };
    let mut prefixDictStart: Ptr<u8> = (dest - dictSize).cast();
    let mut curSrc: Ptr<u8> = src.cast();
    let mut srcEnd: Ptr<u8> = (curSrc + srcSize).cast();
    let mut curDest: Ptr<u8> = dest.cast();
    let mut destEnd: Ptr<u8> = (curDest + outBufferSize).cast();
    let mut srcEndFast: Ptr<u8> = (srcEnd - RAPIDLZ_COPY_PROTECT_SIZE!()).cast();
    let mut destEndFast: Ptr<u8> = (destEnd - RAPIDLZ_COPY_PROTECT_SIZE!()).cast();
    let mut token: u32 = Default::default();
    let mut len: u32 = Default::default();
    let mut offset: u16 = Default::default();
    let mut matchSrc: Ptr<u8> = Default::default();
    let mut tmp: u32 = 0;
    let mut leftSrcSize: usize = Default::default();
    loop {
        token = (*curSrc).cast();
        curSrc += 1;
        len = token >> 4;
        if RAPIDLZ_LIKELY!(len < RAPIDLZ_MAX_4BIT_VALUE!()).as_bool() {
            if RAPIDLZ_LIKELY!((curSrc + len <= srcEndFast).as_bool() && (curDest + len <= destEndFast).as_bool()) {
                RapidlzCopy16Byte(curDest.cast(), curSrc.cast());
                RAPIDLZ_POSITION_UPDATE!(curSrc, curDest, len);
            } else {
                leftSrcSize = (srcEnd - curSrc).cast();
                RAPIDLZ_SAFE_COPY_TILL_END!(curSrc, leftSrcSize, curDest, destEnd, len);
            }
        } else {
            RAPIDLZ_READ_OPTIONAL_LENGTH!(len, curSrc, srcEnd, tmp);
            if RAPIDLZ_LIKELY!((curSrc + len <= srcEndFast).as_bool() && (curDest + len <= destEndFast).as_bool()) {
                RapidlzWildCopy16(curSrc.cast(), curDest.cast(), (curDest + len).cast());
                RAPIDLZ_POSITION_UPDATE!(curSrc, curDest, len);
            } else {
                leftSrcSize = (srcEnd - curSrc).cast();
                RAPIDLZ_SAFE_COPY_TILL_END!(curSrc, leftSrcSize, curDest, destEnd, len);
            }
        }
        offset = RapidlzReadLE16Bit(curSrc.cast());
        curSrc += 2;
        matchSrc = (curDest - offset).cast();
        if RAPIDLZ_UNLIKELY!(matchSrc < prefixDictStart).as_bool() {
            return RAPIDLZ_DEC_NOT_OK!();
        }
        len = token & RAPIDLZ_MAX_4BIT_VALUE!();
        if len < RAPIDLZ_MAX_4BIT_VALUE!() {
            len += 4;
        } else {
            RAPIDLZ_READ_OPTIONAL_LENGTH!(len, curSrc, srcEnd, tmp);
            len += 4;
        }
        if (curDest + len > destEnd - RAPIDLZ_LAST_LITERALS!()).as_bool() {
            return RAPIDLZ_DEC_NOT_OK!();
        }
        if RAPIDLZ_LIKELY!((curDest + len) <= (destEndFast - RAPIDLZ_COPY_PROTECT_SIZE!() + RAPIDLZ_LAST_LITERALS!())).as_bool() {
            RapidlzCopyMatchFast(curDest.cast(), matchSrc.cast(), offset.cast(), len.cast());
            curDest += len;
        } else {
            if RAPIDLZ_LIKELY!(len < 1024).as_bool() {
                RAPIDLZ_FAST_SAFE_COPY_BY_BYTES!(curDest, matchSrc, len);
            } else {
                RapidlzSafeCopyMatchFast(curDest.cast(), matchSrc.cast(), destEnd.cast(), offset.cast(), len.cast());
                curDest += len;
            }
        }
    }
    return (curDest - dest).cast::<i32>();
}
