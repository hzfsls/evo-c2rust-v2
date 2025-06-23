pub fn RapidlzCompressStoreOptionalLength(mut dst: Ptr<u8>, mut litLength: u32) -> Ptr<u8> {
    let mut dstCurr: Ptr<u8> = dst;
    let mut length: u32 = litLength;
    if length < RAPIDLZ_MAX_BYTE_VALUE!() {
        *dstCurr = length.cast();
        dstCurr = dstCurr + 1;
        return dstCurr;
    }
    loop {
        *dstCurr = RAPIDLZ_MAX_BYTE_VALUE!();
        dstCurr = dstCurr + 1;
        length -= RAPIDLZ_MAX_BYTE_VALUE!();
        if length < RAPIDLZ_MAX_BYTE_VALUE!() {
            break;
        }
    }
    *dstCurr = length.cast();
    dstCurr += 1;
    return dstCurr;
}