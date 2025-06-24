pub fn RapidlzCopyMatchFast(
    mut dst: Ptr<u8>,
    mut r#match: Ptr<u8>,
    mut offset: u16,
    mut length: u32,
) {
    let mut dstCurr: Ptr<u8> = dst;
    let mut matchPtr: Ptr<u8> = r#match;
    if offset >= RAPIDLZ_SIXTEEN_BYTE!() {
        RapidlzCopyLiteralsFast(matchPtr, dstCurr, length);
        return;
    }
    c_for!(let mut i = 0; i < RAPIDLZ_EIGHT_BYTE!(); i += 1; {
        dstCurr[i] = matchPtr[i];
    });
    if length <= RAPIDLZ_EIGHT_BYTE!() {
        return;
    }
    let mut dstEnd: Ptr<u8> = dstCurr + length;
    if offset < RAPIDLZ_EIGHT_BYTE!() {
        matchPtr += g_overlapOffAddVal.lock()[offset];
        dstCurr += RAPIDLZ_EIGHT_BYTE!();
    }
    loop {
        RapidlzCopy8Byte(dstCurr, matchPtr);
        dstCurr += RAPIDLZ_EIGHT_BYTE!();
        matchPtr += RAPIDLZ_EIGHT_BYTE!();
        if dstCurr >= dstEnd {
            break;
        }
    }
}