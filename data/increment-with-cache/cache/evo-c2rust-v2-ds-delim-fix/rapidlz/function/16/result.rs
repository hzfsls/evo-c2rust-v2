pub fn RapidlzDecWithExternalDict(mut src: Ptr<u8>, mut dest: Ptr<u8>, mut srcSize: i32, mut outBufferSize: i32, mut dictStart: Ptr<u8>, mut dictSize: i32) -> i32 {
    let mut curSrc: Ptr<u8> = src.cast();
    let mut srcEnd: Ptr<u8> = (curSrc + srcSize).cast();
    let mut curDest: Ptr<u8> = dest.cast();
    let mut destEnd: Ptr<u8> = (curDest + outBufferSize).cast();
    let mut srcEndFast: Ptr<u8> = (srcEnd - RAPIDLZ_COPY_PROTECT_SIZE!()).cast();
    let mut destEndFast: Ptr<u8> = (destEnd - RAPIDLZ_COPY_PROTECT_SIZE!()).cast();
    let mut dictEnd: Ptr<u8> = (dictStart.cast::<Ptr<u8>>() + dictSize).cast();

    let mut token: u32;
    let mut len: u32;
    let mut offset: u16;
    let mut matchSrc: Ptr<u8>;
    let mut temp: u32 = 0;
    let mut leftSrcSize: usize;
    loop {
        token = (*curSrc).cast();
        curSrc += 1;

        len = token >> 4;
        if RAPIDLZ_LIKELY!(len < RAPIDLZ_MAX_4BIT_VALUE!()).as_bool() {
            if RAPIDLZ_LIKELY!(RAPIDLZ_DICT_FAST_COPY_AVAIL!(curSrc, len, srcEndFast, curDest, destEndFast)).as_bool() {
                RapidlzCopy16Byte(curDest.cast(), curSrc.cast());
                RAPIDLZ_POSITION_UPDATE!(curSrc, curDest, len);
            } else {
                leftSrcSize = (srcEnd - curSrc).cast();
                RAPIDLZ_SAFE_COPY_TILL_END!(curSrc, leftSrcSize, curDest, destEnd, len);
            }
        } else {
            RAPIDLZ_READ_OPTIONAL_LENGTH!(len, curSrc, srcEnd, temp);
            if RAPIDLZ_LIKELY!(RAPIDLZ_DICT_FAST_COPY_AVAIL!(curSrc, len, srcEndFast, curDest, destEndFast)).as_bool() {
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

        len = token & RAPIDLZ_MAX_4BIT_VALUE!();

        RAPIDLZ_GET_MATCH_LEN!(len, curSrc, srcEnd, temp);

        if (matchSrc >= dest.cast::<Ptr<u8>>()).as_bool() {
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
        } else {
            let mut err: errno_t;
            if (len.cast::<i32>() <= (dest.cast::<Ptr<u8>>() - matchSrc).cast::<i32>()).as_bool() {
                err = c_memmove_s!(curDest, destEnd - curDest, dictEnd - (dest.cast::<Ptr<u8>>() - matchSrc), len);
                curDest += len;
            } else {
                let mut externCopySize: usize = (dest.cast::<Ptr<u8>>() - matchSrc).cast();
                let mut innerCopySize: usize = len - externCopySize;
                err = c_memcpy_s!(curDest, destEnd - curDest, dictEnd - externCopySize, externCopySize);
                curDest += externCopySize;
                if (innerCopySize > (curDest - dest.cast::<Ptr<u8>>()).cast::<usize>()).as_bool() {
                    let mut copySrc: Ptr<u8> = dest.cast();
                    while (innerCopySize.suffix_minus_minus() != 0).as_bool() {
                        *curDest = *copySrc;
                        curDest += 1;
                        copySrc += 1;
                    }
                } else {
                    err = c_memcpy_s!(curDest, destEnd - curDest, dest.cast(), innerCopySize);
                    curDest += innerCopySize;
                }
            }
        }
    }

    return (curDest.cast::<Ptr<Void>>() - dest.cast::<Ptr<Void>>()).cast::<i32>();
}
