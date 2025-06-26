pub fn RapidlzStoreLastLiterals(mut dst: Ptr<u8>, mut dstEnd: Ptr<u8>, mut srcCurr: Ptr<u8>, mut litLength: u32, mut bufferLimit: u8) -> Ptr<u8> {
    let mut dstCurr: Ptr<u8> = dst.cast();
    if (bufferLimit != 0).as_bool() {
        let mut litTokSize: u32 = 1 + litLength + (litLength / RAPIDLZ_MAX_BYTE_VALUE!());
        if (dstCurr + litTokSize > dstEnd).as_bool() {
            RAPIDLZ_LOG!(RAPIDLZ_DST_SIZE_SMALL!(), cstr!("dstEnd - dstCur:{} litTokSize:{}\n"), dstEnd - dstCurr, litTokSize);
            return NULL!();
        }
    }
    let mut token: u8 = if litLength < RAPIDLZ_MAX_4BIT_VALUE!() { litLength.cast() } else { RAPIDLZ_MAX_4BIT_VALUE!() } << 4;
    *dstCurr = token.cast();
    dstCurr += 1;
    if (litLength >= RAPIDLZ_MAX_4BIT_VALUE!()).as_bool() {
        dstCurr = RapidlzCompressStoreOptionalLength(dstCurr.cast(), (litLength - RAPIDLZ_MAX_4BIT_VALUE!()).cast()).cast();
    }
    if c_memcpy_s!(dstCurr, dstEnd - dstCurr, srcCurr, litLength) != EOK!() {
        RAPIDLZ_LOG!(RAPIDLZ_SECUREC_ERROR!(), cstr!("dstEnd - dstCurr:{} litLength{}\n"), dstEnd - dstCurr, litLength);
        return NULL!();
    }
    return (dstCurr + litLength).cast();
}
