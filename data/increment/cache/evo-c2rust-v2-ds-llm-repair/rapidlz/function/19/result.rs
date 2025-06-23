pub fn RapidlzDecompress(mut src: Ptr<Void>, mut dst: Ptr<Void>, mut srcSize: usize, mut dstSize: usize) -> usize {
    if (src == NULL!()) || (dst == NULL!()) || (srcSize == 0) || (dstSize == 0) {
        RAPIDLZ_LOG!(RAPIDLZ_INPUT_INVALID!(), cstr!("input invalid\n"));
        return 0;
    }

    let mut token: u8 = Default::default();
    let mut temp: u32 = 0;
    let mut offset: u16 = Default::default();
    let mut litLen: u32 = Default::default();
    let mut matchLen: u32 = Default::default();
    let mut matchSrc: Ptr<u8> = Default::default();
    let mut srcEnd: Ptr<u8> = (src.cast::<Ptr<u8>>() + srcSize);
    let mut srcCurr: Ptr<u8> = src.cast::<Ptr<u8>>();
    let mut srcEndFast: Ptr<u8> = (srcEnd - RAPIDLZ_COPY_PROTECT_SIZE!());
    let mut dstEnd: Ptr<u8> = (dst.cast::<Ptr<u8>>() + dstSize);
    let mut dstCurr: Ptr<u8> = dst.cast::<Ptr<u8>>();
    let mut dstEndFast: Ptr<u8> = (dstEnd - RAPIDLZ_COPY_PROTECT_SIZE!());

    while (srcCurr < srcEnd) {
        token = *srcCurr;
        srcCurr += 1;
        litLen = (token >> 4).cast();

        if RAPIDLZ_LIKELY!(litLen < RAPIDLZ_MAX_4BIT_VALUE!()) {
            if RAPIDLZ_LIKELY!(srcCurr + litLen as usize <= srcEndFast && dstCurr + litLen as usize <= dstEndFast) {
                RapidlzCopy16Byte(dstCurr, srcCurr);
                dstCurr += litLen as usize;
                srcCurr += litLen as usize;
                continue;
            }
        } else {
            RAPIDLZ_READ_OPTIONAL_LENGTH!(litLen, srcCurr, srcEnd, temp);
            if RAPIDLZ_LIKELY!(srcCurr + litLen as usize <= srcEndFast && dstCurr + litLen as usize <= dstEndFast) {
                RapidlzWildCopy16(srcCurr, dstCurr, (dstCurr + litLen as usize));
                dstCurr += litLen as usize;
                srcCurr += litLen as usize;
                continue;
            }
        }

        let mut leftSrcSize: usize = (srcEnd - srcCurr);
        if RAPIDLZ_UNLIKELY!(litLen as usize > leftSrcSize || c_memmove_s!(dstCurr, dstEnd - dstCurr, srcCurr, litLen as usize) != EOK!()) {
            RAPIDLZ_LOG!(RAPIDLZ_DST_SIZE_SMALL!(), cstr!("litLen:{} dstEnd - dst:{}\n"), litLen, leftSrcSize);
            return 0;
        }

        dstCurr += litLen as usize;
        srcCurr += litLen as usize;

        if (leftSrcSize == litLen as usize) {
            return (dstCurr - dst.cast::<Ptr<u8>>());
        }

        if RAPIDLZ_UNLIKELY!(srcCurr > srcEnd - 2) {
            RAPIDLZ_LOG!(RAPIDLZ_FORMAT_INVALID!(), cstr!("rapidlz format invalid\n"));
            return 0;
        }
        offset = RapidlzReadLE16Bit(srcCurr);
        srcCurr += 2;
        matchSrc = (dstCurr - offset as usize);
        if RAPIDLZ_UNLIKELY!(matchSrc.cast::<Ptr<Void>>() < dst) {
            RAPIDLZ_LOG!(RAPIDLZ_FORMAT_INVALID!(), cstr!("rapidlz format invalid\n"));
            return 0;
        }

        matchLen = (token & RAPIDLZ_MAX_4BIT_VALUE!()).cast::<u32>() + RAPIDLZ_MIN_MATCH!();
        if (matchLen == RAPIDLZ_MAX_4BIT_MATCH!()) {
            RAPIDLZ_READ_OPTIONAL_LENGTH!(matchLen, srcCurr, srcEnd, temp);
        }

        if RAPIDLZ_LIKELY!(dstCurr + matchLen as usize <= dstEndFast) {
            RapidlzCopyMatchFast(dstCurr, matchSrc, offset, matchLen);
            dstCurr += matchLen as usize;
        } else {
            if (dstCurr + matchLen as usize > dstEnd) {
                RAPIDLZ_LOG!(RAPIDLZ_DST_SIZE_SMALL!(), cstr!("dstEnd - dstCurr:{} matchLen:{}\n"), (dstEnd - dstCurr), matchLen);
                return 0;
            }

            SAFE_COPY_MATCH!(dstCurr, matchSrc, matchLen as usize);
        }
    }

    return (dstCurr - dst.cast::<Ptr<u8>>());
}
