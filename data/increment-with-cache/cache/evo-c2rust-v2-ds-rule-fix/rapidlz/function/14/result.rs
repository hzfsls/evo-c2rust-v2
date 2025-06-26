pub fn RapidlzSafeCopyMatchFast(mut curDest: Ptr<u8>, mut matchSrc: Ptr<u8>, mut destEnd: Ptr<u8>, mut offset: u16, mut len: u32) {
    let mut err: errno_t = Default::default();
    let mut curDestCopyEnd: Ptr<u8> = (curDest + len);
    if (offset < RAPIDLZ_EIGHT_BYTE!()) {
        curDest[0] = matchSrc[0];
        curDest[1] = matchSrc[1];
        curDest[2] = matchSrc[2];
        curDest[3] = matchSrc[3];
        matchSrc += (*g_enc32table.lock())[offset];
        err = c_memcpy_s!((curDest + RAPIDLZ_FOUR_BYTE!()), RAPIDLZ_FOUR_BYTE!(), matchSrc, RAPIDLZ_FOUR_BYTE!());
        matchSrc -= (*g_dec64table.lock())[offset];
    } else {
        err = c_memcpy_s!(curDest, RAPIDLZ_EIGHT_BYTE!(), matchSrc, RAPIDLZ_EIGHT_BYTE!());
        matchSrc += RAPIDLZ_EIGHT_BYTE!();
    }
    #[cfg(RAPIDLZ_DEBUG)]
    RAPIDLZ_RETURN_IF_NOT_EOK!(err, RAPIDLZ_DEC_NOT_OK!());
    #[cfg(not(RAPIDLZ_DEBUG))]
    err.cast::<Void>();
    curDest += RAPIDLZ_EIGHT_BYTE!();
    let mut curDestLimit: Ptr<u8> = (destEnd - (RAPIDLZ_EIGHT_BYTE!() - 1));
    if (curDest < curDestLimit) {
        RapidlzWildCopy8(matchSrc, curDest, curDestLimit);
        matchSrc += (curDestLimit - curDest).cast::<u32>();
        curDest = curDestLimit;
    }
    while (curDest < curDestCopyEnd) {
        *curDest = *matchSrc;
        curDest += 1;
        matchSrc += 1;
    }
}
