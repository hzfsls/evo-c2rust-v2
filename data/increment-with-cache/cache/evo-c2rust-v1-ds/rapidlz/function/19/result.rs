pub fn RapidlzDecompress(mut src: Ptr<Void>, mut dst: Ptr<Void>, mut srcSize: usize, mut dstSize: usize) -> usize {
    if src == NULL!() || dst == NULL!() || srcSize == 0 || dstSize == 0 {
        RAPIDLZ_LOG!(RAPIDLZ_INPUT_INVALID!(), cstr!("input invalid\n"));
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
    let mut srcEndFast: Ptr<u8> = srcEnd - RAPIDLZ_COPY_PROTECT_SIZE!();
    let mut dstEnd: Ptr<u8> = (dst.cast::<Ptr<u8>>() + dstSize).cast();
    let mut dstCurr: Ptr<u8> = dst.cast::<Ptr<u8>>();
    let mut dstEndFast: Ptr<u8> = dstEnd - RAPIDLZ_COPY_PROTECT_SIZE!();

    while srcCurr < srcEnd {
        token = *srcCurr;
        srcCurr += 1;
        litLen = (token >> 4).cast();

        if RAPIDLZ_LIKELY!(litLen < RAPIDLZ_MAX_4BIT_VALUE!()) {
            if RAPIDLZ_LIKELY!(srcCurr + litLen <= srcEndFast && dstCurr + litLen <= dstEndFast) {
                RapidlzCopy16Byte(dstCurr.cast(), srcCurr.cast());
                dstCurr += litLen;
                srcCurr += litLen;
                goto!(READ_MATCH);
            }
        } else {
            RAPIDLZ_READ_OPTIONAL_LENGTH!(litLen, srcCurr, srcEnd, temp);
            if RAPIDLZ_LIKELY!(srcCurr + litLen <= srcEndFast && dstCurr + litLen <= dstEndFast) {
                RapidlzWildCopy16(srcCurr.cast(), dstCurr.cast(), (dstCurr + litLen).cast());
                dstCurr += litLen;
                srcCurr += litLen;
                goto!(READ_MATCH);
            }
        }

        let mut leftSrcSize: usize = srcEnd - srcCurr;
        if RAPIDLZ_UNLIKELY!(litLen > leftSrcSize || c_memmove_s!(dstCurr, dstEnd - dstCurr, srcCurr, litLen) != EOK!()) {
            RAPIDLZ_LOG!(RAPIDLZ_DST_SIZE_SMALL!(), cstr!("litLen:{} dstEnd - dst:{}\n"), litLen, leftSrcSize);
            return 0;
        }

        dstCurr += litLen;
        srcCurr += litLen;

        if leftSrcSize == litLen {
            return (dstCurr - dst.cast::<Ptr<u8>>()).cast();
        }

        label!(READ_MATCH);
        if RAPIDLZ_UNLIKELY!(srcCurr > srcEnd - 2) {
            RAPIDLZ_LOG!(RAPIDLZ_FORMAT_INVALID!(), cstr!("rapidlz format invalid\n"));
            return 0;
        }
        offset = RapidlzReadLE16Bit(srcCurr.cast());
        srcCurr += 2;
        matchSrc = dstCurr - offset;
        if RAPIDLZ_UNLIKELY!(matchSrc.cast::<Ptr<Void>>() < dst) {
            RAPIDLZ_LOG!(RAPIDLZ_FORMAT_INVALID!(), cstr!("rapidlz format invalid\n"));
            return 0;
        }

        matchLen = (token & RAPIDLZ_MAX_4BIT_VALUE!()).cast::<u32>() + RAPIDLZ_MIN_MATCH!();
        if matchLen == RAPIDLZ_MAX_4BIT_MATCH!() {
            RAPIDLZ_READ_OPTIONAL_LENGTH!(matchLen, srcCurr, srcEnd, temp);
        }

        if RAPIDLZ_LIKELY!(dstCurr + matchLen <= dstEndFast) {
            RapidlzCopyMatchFast(dstCurr.cast(), matchSrc.cast(), offset.cast(), matchLen.cast());
            dstCurr += matchLen;
        } else {
            if dstCurr + matchLen > dstEnd {
                RAPIDLZ_LOG!(RAPIDLZ_DST_SIZE_SMALL!(), cstr!("dstEnd - dstCurr:{} matchLen:{}\n"), dstEnd - dstCurr, matchLen);
                return 0;
            }

            SAFE_COPY_MATCH!(dstCurr, matchSrc, matchLen);
        }
    }

    return (dstCurr - dst.cast::<Ptr<u8>>()).cast();
}
