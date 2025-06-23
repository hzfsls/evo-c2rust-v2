pub fn RapidlzStoreOffMatch(mut dst: Ptr<u8>, mut token: Ptr<u8>, mut matchLength: u32, mut offset: u16) -> Ptr<u8> {
    let mut dstCurr: Ptr<u8> = dst.cast();
    RapidlzWriteLE16(dstCurr.cast(), offset.cast());
    dstCurr += 2;
    if (matchLength >= RAPIDLZ_MAX_4BIT_VALUE!()).as_bool() {
        let mut optionalLen: u32 = matchLength - RAPIDLZ_MAX_4BIT_VALUE!();
        *token += RAPIDLZ_MAX_4BIT_VALUE!();
        c_for!(; optionalLen >= RAPIDLZ_MAX_BYTE_VALUE!(); optionalLen -= RAPIDLZ_MAX_BYTE_VALUE!(); {
            *dstCurr = RAPIDLZ_MAX_BYTE_VALUE!();
            dstCurr += 1;
        });
        *dstCurr = optionalLen.cast::<u8>();
        dstCurr += 1;
    } else {
        *token += matchLength.cast::<u8>();
    }
    return dstCurr.cast();
}
