pub fn RapidlzCompressStoreOptionalLength(mut dst: Ptr<u8>, mut litLength: u32) -> Ptr<u8> {
    let mut dstCurr: Ptr<u8> = dst.cast();
    let mut length: u32 = litLength.cast();

    if (length < RAPIDLZ_MAX_BYTE_VALUE!()).as_bool() {
        *dstCurr = length.cast::<u8>();
        dstCurr += 1;
        return dstCurr.cast();
    }

    c_do!({
        *dstCurr = RAPIDLZ_MAX_BYTE_VALUE!();
        dstCurr += 1;
        length -= RAPIDLZ_MAX_BYTE_VALUE!();
    } while length >= RAPIDLZ_MAX_BYTE_VALUE!());

    *dstCurr = length.cast::<u8>();
    dstCurr += 1;
    return dstCurr.cast();
}
