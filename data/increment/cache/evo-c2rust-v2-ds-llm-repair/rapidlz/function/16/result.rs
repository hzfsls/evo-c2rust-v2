pub fn RapidlzDecWithExternalDict(mut src: Ptr<u8>, mut dest: Ptr<u8>, mut srcSize: i32, mut outBufferSize: i32, mut dictStart: Ptr<u8>, mut dictSize: i32) -> i32 {
    let mut curSrc: Ptr<u8> = src;
    let mut srcEnd: Ptr<u8> = (curSrc + srcSize.cast::<usize>());
    let mut curDest: Ptr<u8> = dest;
    let mut destEnd: Ptr<u8> = (curDest + outBufferSize.cast::<usize>());
    let mut srcEndFast: Ptr<u8> = (srcEnd - RAPIDLZ_COPY_PROTECT_SIZE);
    let mut destEndFast: Ptr<u8> = (destEnd - RAPIDLZ_COPY_PROTECT_SIZE);
    let mut dictEnd: Ptr<u8> = (dictStart + dictSize.cast::<usize>());

    let mut token: u32;
    let mut len: u32;
    let mut offset: u16;
    let mut matchSrc: Ptr<u8>;
    let mut temp: u32 = 0;
    let mut leftSrcSize: usize;
    loop {
        token = (*curSrc).cast::<u32>();
        curSrc += 1;

        len = token >> 4;
        if RAPIDLZ_LIKELY!(len < RAPIDLZ_MAX_4BIT_VALUE) {
            if RAPIDLZ_LIKELY!(RAPIDLZ_DICT_FAST_COPY_AVAIL!(curSrc, len, srcEndFast, curDest, destEndFast)) {
                RapidlzCopy16Byte(curDest, curSrc);
                RAPIDLZ_POSITION_UPDATE!(curSrc, curDest, len);
            } else {
                leftSrcSize = (srcEnd - curSrc).cast::<usize>();
                RAPIDLZ_SAFE_COPY_TILL_END!(curSrc, leftSrcSize, curDest, destEnd, len.cast::<usize>());
            }
        } else {
            RAPIDLZ_READ_OPTIONAL_LENGTH!(len, curSrc, srcEnd, temp);
            if RAPIDLZ_LIKELY!(RAPIDLZ_DICT_FAST_COPY_AVAIL!(curSrc, len, srcEndFast, curDest, destEndFast)) {
                RapidlzWildCopy16(curSrc, curDest, (curDest + len.cast::<usize>()));
                RAPIDLZ_POSITION_UPDATE!(curSrc, curDest, len);
            } else {
                leftSrcSize = (srcEnd - curSrc).cast::<usize>();
                RAPIDLZ_SAFE_COPY_TILL_END!(curSrc, leftSrcSize, curDest, destEnd, len.cast::<usize>());
            }
        }

        offset = RapidlzReadLE16Bit(curSrc);
        curSrc += 2;
        matchSrc = (curDest - offset.cast::<usize>());

        len = token & RAPIDLZ_MAX_4BIT_VALUE;

        RAPIDLZ_GET_MATCH_LEN!(len, curSrc, srcEnd, temp);

        if (matchSrc >= dest) {
            if RAPIDLZ_LIKELY!((curDest + len.cast::<usize>()) <= (destEndFast - RAPIDLZ_COPY_PROTECT_SIZE + RAPIDLZ_LAST_LITERALS)) {
                RapidlzCopyMatchFast(curDest, matchSrc, offset, len);
                curDest += len.cast::<usize>();
            } else {
                if RAPIDLZ_LIKELY!(len < 1024) {
                    RAPIDLZ_FAST_SAFE_COPY_BY_BYTES!(curDest, matchSrc, len.cast::<usize>());
                } else {
                    RapidlzSafeCopyMatchFast(curDest, matchSrc, destEnd, offset, len);
                    curDest += len.cast::<usize>();
                }
            }
        } else {
            let mut err: errno_t;
            if (len.cast::<i32>() <= (dest - matchSrc).cast::<i32>()) {
                err = c_memmove_s!(curDest, (destEnd - curDest).cast::<usize>(), (dictEnd - (dest - matchSrc)).cast::<Ptr<Void>>(), len.cast::<usize>());
                curDest += len.cast::<usize>();
            } else {
                let mut externCopySize: usize = (dest - matchSrc).cast::<usize>();
                let mut innerCopySize: usize = (len.cast::<usize>() - externCopySize);
                err = c_memcpy_s!(curDest, (destEnd - curDest).cast::<usize>(), (dictEnd - externCopySize).cast::<Ptr<Void>>(), externCopySize);
                curDest += externCopySize;
                if (innerCopySize > (curDest - dest).cast::<usize>()) {
                    let mut copySrc: Ptr<u8> = dest;
                    while (innerCopySize.suffix_minus_minus() != 0) {
                        *curDest = *copySrc;
                        curDest += 1;
                        copySrc += 1;
                    }
                } else {
                    err = c_memcpy_s!(curDest, (destEnd - curDest).cast::<usize>(), dest.cast::<Ptr<Void>>(), innerCopySize);
                    curDest += innerCopySize;
                }
            }
        }
    }

    return (curDest - dest).cast::<i32>();
}
