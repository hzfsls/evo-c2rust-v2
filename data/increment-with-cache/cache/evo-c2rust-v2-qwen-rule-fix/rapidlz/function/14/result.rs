pub fn RapidlzSafeCopyMatchFast(mut curDest: Ptr<u8>, mut matchSrc: Ptr<u8>, mut destEnd: Ptr<u8>, mut offset: u16, mut len: u32) {
    let mut err: errno_t;
    let mut curDestCopyEnd: Ptr<u8> = (curDest + len);
    if (offset < RAPIDLZ_EIGHT_BYTE!()) {
        curDest[0] = matchSrc[0];
        curDest[1] = matchSrc[1];
        curDest[2] = matchSrc[2];
        curDest[3] = matchSrc[3];
        matchSrc += (*g_enc32table.lock())[offset];
        err = c_memcpy_s!(curDest + RAPIDLZ_FOUR_BYTE!(), RAPIDLZ_FOUR_BYTE!(), matchSrc, RAPIDLZ_FOUR_BYTE!());
        matchSrc -= (*g_dec64table.lock())[offset];
    } else {
        err = c_memcpy_s!(curDest, RAPIDLZ_EIGHT_BYTE!(), matchSrc, RAPIDLZ_EIGHT_BYTE!());
        matchSrc += RAPIDLZ_EIGHT_BYTE!();
    }
    if RAPIDLZ_DEBUG!() {
        if (err != EOK!()) {
            return RAPIDLZ_DEC_NOT_OK!();
        }
    } else {
        err = 0;
    }
    curDest += RAPIDLZ_EIGHT_BYTE!();
    let mut curDestLimit: Ptr<u8> = (destEnd - (RAPIDLZ_EIGHT_BYTE!() - 1));
    if (curDest < curDestLimit) {
        RapidlzWildCopy8(matchSrc, curDest, curDestLimit);
        matchSrc += (curDestLimit - curDest);
        curDest = curDestLimit;
    }
    while (curDest < curDestCopyEnd) {
        *curDest = *matchSrc;
        curDest += 1;
        matchSrc += 1;
    }
}