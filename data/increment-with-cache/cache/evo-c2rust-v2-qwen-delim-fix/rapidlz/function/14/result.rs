pub fn RapidlzSafeCopyMatchFast(mut curDest: Ptr<u8>, mut matchSrc: Ptr<u8>, mut destEnd: Ptr<u8>, mut offset: u16, mut len: u32) {
    let mut err: errno_t;
    let mut curDestCopyEnd: Ptr<u8> = (curDest + len).cast();
    if (offset < RAPIDLZ_EIGHT_BYTE!()).as_bool() {
        curDest[0] = matchSrc[0].cast();
        curDest[1] = matchSrc[1].cast();
        curDest[2] = matchSrc[2].cast();
        curDest[3] = matchSrc[3].cast();
        matchSrc += (*g_enc32table.lock())[offset];
        err = c_memcpy_s!(curDest + RAPIDLZ_FOUR_BYTE!(), RAPIDLZ_FOUR_BYTE!(), matchSrc.cast(), RAPIDLZ_FOUR_BYTE!());
        matchSrc -= (*g_dec64table.lock())[offset];
    } else {
        err = c_memcpy_s!(curDest.cast(), RAPIDLZ_EIGHT_BYTE!(), matchSrc.cast(), RAPIDLZ_EIGHT_BYTE!());
        matchSrc += RAPIDLZ_EIGHT_BYTE!();
    }
    if RAPIDLZ_DEBUG!().as_bool() {
        if (err != EOK!()).as_bool() {
            return RAPIDLZ_DEC_NOT_OK!();
        }
    } else {
        err = 0;
    }
    curDest += RAPIDLZ_EIGHT_BYTE!();
    let mut curDestLimit: Ptr<u8> = (destEnd - (RAPIDLZ_EIGHT_BYTE!() - 1)).cast();
    if (curDest < curDestLimit).as_bool() {
        RapidlzWildCopy8(matchSrc.cast(), curDest.cast(), curDestLimit.cast());
        matchSrc += (curDestLimit - curDest).cast();
        curDest = curDestLimit.cast();
    }
    while (curDest < curDestCopyEnd).as_bool() {
        *curDest = *matchSrc;
        curDest += 1;
        matchSrc += 1;
    }
}