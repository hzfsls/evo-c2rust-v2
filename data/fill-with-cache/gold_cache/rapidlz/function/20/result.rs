pub fn RapidlzStoreLastLiterals(
    mut dst: Ptr<u8>,
    mut dstEnd: Ptr<u8>,
    mut srcCurr: Ptr<u8>,
    mut litLength: u32,
    mut bufferLimit: u8,
) -> Ptr<u8> {
    let mut dstCurr: Ptr<u8> = dst;
    if bufferLimit != 0 {
        let litTokSize: u32 = 1 + litLength + (litLength / RAPIDLZ_MAX_BYTE_VALUE!());
        if dstCurr + litTokSize > dstEnd {
            RAPIDLZ_LOG!(
                RAPIDLZ_DST_SIZE_SMALL!(),
                cstr!("dstEnd - dstCur:{} litTokSize:{}"),
                dstEnd - dstCurr,
                litTokSize
            );
            return NULL!();
        }
    }
    let token: u8 = (if litLength < RAPIDLZ_MAX_4BIT_VALUE!() {
        litLength
    } else {
        RAPIDLZ_MAX_4BIT_VALUE!()
    } << 4)
        .cast();
    *dstCurr = token;
    dstCurr += 1;
    if litLength >= RAPIDLZ_MAX_4BIT_VALUE!() {
        dstCurr =
            RapidlzCompressStoreOptionalLength(dstCurr, litLength - RAPIDLZ_MAX_4BIT_VALUE!());
    }
    if c_memcpy_s!(dstCurr, dstEnd - dstCurr, srcCurr, litLength) != EOK!() {
        RAPIDLZ_LOG!(
            RAPIDLZ_SECUREC_ERROR!(),
            cstr!("dstEnd - dstCurr:{} litLength:{}"),
            dstEnd - dstCurr,
            litLength
        );
        return NULL!();
    }
    return dstCurr + litLength;
}