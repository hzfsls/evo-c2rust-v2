pub fn RapidlzStoreLastLiterals(mut dst: Ptr<u8>, mut dstEnd: Ptr<u8>, mut srcCurr: Ptr<u8>, mut litLength: u32, mut bufferLimit: u8) -> Ptr<u8> {
    let mut dstCurr: Ptr<u8> = dst.cast();
    if bufferLimit != 0 {
        let mut litTokSize: u32 = 1 + litLength + (litLength / RAPIDLZ_MAX_BYTE_VALUE!()).cast();
        if (dstCurr + litTokSize) > dstEnd {
            RAPIDLZ_LOG!(RAPIDLZ_DST_SIZE_SMALL, cstr!("dstEnd - dstCur:{} litTokSize:{}\n"), (dstEnd - dstCurr).cast(), litTokSize.cast());
            return NULL!();
        }
    }
    let mut token: u8 = if litLength < RAPIDLZ_MAX_4BIT_VALUE!() { litLength.cast() } else { RAPIDLZ_MAX_4BIT_VALUE!().cast() };
    token = (token << 4).cast();
    *dstCurr = token;
    dstCurr += 1;
    if litLength >= RAPIDLZ_MAX_4BIT_VALUE!() {
        dstCurr = RapidlzCompressStoreOptionalLength(dstCurr.cast(), (litLength - RAPIDLZ_MAX_4BIT_VALUE!()).cast()).cast();
    }
    let mut ret: i32 = c_memcpy_s!(dstCurr.cast(), (dstEnd - dstCurr).cast(), srcCurr.cast(), litLength.cast());
    if (ret != EOK!()).as_bool() {
        RAPIDLZ_LOG!(RAPIDLZ_SECUREC_ERROR, cstr!("dstEnd - dstCurr:{} litLength:{}\n"), (dstEnd - dstCurr).cast(), litLength.cast());
        return NULL!();
    }
    return (dstCurr + litLength).cast();
}