pub fn RapidlzDecompress(mut src: Ptr<Void>, mut dst: Ptr<Void>, mut srcSize: usize, mut dstSize: usize) -> usize {
    if (src == NULL!()).as_bool() || (dst == NULL!()).as_bool() || (srcSize == 0).as_bool() || (dstSize == 0).as_bool() {
        RAPIDLZ_LOG!(RAPIDLZ_INPUT_INVALID, cstr!("input invalid\n"));
        return 0;
    }

    let mut token: u8 = Default::default();
    let mut temp: u8 = 0;
    let mut offset: u16 = Default::default();
    let mut litLen: u32 = Default::default();
    let mut matchLen: u32 = Default::default();
    let mut matchSrc: Ptr<u8> = Default::default();
    let mut srcEnd: Ptr<u8> = (src.cast::<Ptr<u8>>() + srcSize).cast();
    let mut srcCurr: Ptr<u8> = src.cast::<Ptr<u8>>();
    let mut srcEndFast: Ptr<u8> = (srcEnd - RAPIDLZ_COPY_PROTECT_SIZE!()).cast();
    let mut dstEnd: Ptr<u8> = (dst.cast::<Ptr<u8>>() + dstSize).cast();
    let mut dstCurr: Ptr<u8> = dst.cast::<Ptr<u8>>();
    let mut dstEndFast: Ptr<u8> = (dstEnd - RAPIDLZ_COPY_PROTECT_SIZE!()).cast();

    while (srcCurr < srcEnd).as_bool() {
        token = *srcCurr;
        srcCurr += 1;
        litLen = (token >> 4).cast();

        if RAPIDLZ_LIKELY!(litLen < RAPIDLZ_MAX_4BIT_VALUE!()).as_bool() {
            if RAPIDLZ_LIKELY!((srcCurr + litLen) <= srcEndFast && (dstCurr + litLen) <= dstEndFast).as_bool() {
                RapidlzCopy16Byte(dstCurr.cast(), srcCurr.cast());
                dstCurr += litLen;
                srcCurr += litLen;
                goto READ_MATCH;
            }
        } else {
            RAPIDLZ_READ_OPTIONAL_LENGTH!(litLen, srcCurr, srcEnd, temp);
            if RAPIDLZ_LIKELY!((srcCurr + litLen) <= srcEndFast && (dstCurr + litLen) <= dstEndFast).as_bool() {
                RapidlzWildCopy16(srcCurr.cast(), dstCurr.cast(), (dstCurr + litLen).cast());
                dstCurr += litLen;
                srcCurr += litLen;
                goto READ_MATCH;
            }
        }

        let mut leftSrcSize: usize = (srcEnd - srcCurr).cast();
        if RAPIDLZ_UNLIKELY!(litLen > leftSrcSize || c_memmove_s!(dstCurr, (dstEnd - dstCurr).cast(), srcCurr, litLen).cast::<i32>() != EOK!()).as_bool() {
            RAPIDLZ_LOG!(RAPIDLZ_DST_SIZE_SMALL, cstr!("litLen:%u dstEnd - dst:%zu\n"), litLen.cast(), leftSrcSize.cast());
            return 0;
        }

        dstCurr += litLen;
        srcCurr += litLen;

        if (leftSrcSize == litLen).as_bool() {
            return (dstCurr - dst.cast::<Ptr<u8>>()).cast();
        }

    READ_MATCH:
        if RAPIDLZ_UNLIKELY!(srcCurr > (srcEnd - 2)).as_bool() {
            RAPIDLZ_LOG!(RAPIDLZ_FORMAT_INVALID, cstr!("rapidlz format invalid\n"));
            return 0;
        }
        offset = RapidlzReadLE16Bit(srcCurr.cast()).cast();
        srcCurr += 2;
        matchSrc = (dstCurr - offset).cast();
        if RAPIDLZ_UNLIKELY!((*matchSrc.cast::<Ptr<Void>>()) < dst).as_bool() {
            RAPIDLZ_LOG!(RAPIDLZ_FORMAT_INVALID, cstr!("rapidlz format invalid\n"));
            return 0;
        }

        matchLen = (token & RAPIDLZ_MAX_4BIT_VALUE!()).cast() + RAPIDLZ_MIN_MATCH!();
        if (matchLen == RAPIDLZ_MAX_4BIT_MATCH!()).as_bool() {
            RAPIDLZ_READ_OPTIONAL_LENGTH!(matchLen, srcCurr, srcEnd, temp);
        }

        if RAPIDLZ_LIKELY!((dstCurr + matchLen) <= dstEndFast).as_bool() {
            RapidlzCopyMatchFast(dstCurr.cast(), matchSrc.cast(), offset.cast(), matchLen.cast());
            dstCurr += matchLen;
        } else {
            if (dstCurr + matchLen > dstEnd).as_bool() {
                RAPIDLZ_LOG!(RAPIDLZ_DST_SIZE_SMALL, cstr!("dstEnd - dstCurr:%zu matchLen:%u\n"), (dstEnd - dstCurr).cast(), matchLen.cast());
                return 0;
            }

            SAFE_COPY_MATCH!(dstCurr.cast(), matchSrc.cast(), matchLen.cast());
        }
    }

    return (dstCurr - dst.cast::<Ptr<u8>>()).cast();
}