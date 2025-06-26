pub fn BzpBwtDecodeInit(mut blockSize: i32) -> Ptr<BzpBwtDecodeInfo> {
    if BZP_INVALID_BLOCK_SIZE!(blockSize).as_bool() {
        return NULL!();
    }
    let mut bwt: Ptr<BzpBwtDecodeInfo> = c_malloc!(c_sizeof!(BzpBwtDecodeInfo));
    if (bwt == NULL!()).as_bool() {
        return NULL!();
    }
    let mut spaceSize: i32 = BZP_BASE_BLOCK_SIZE!() * blockSize;
    bwt.block = c_malloc!(spaceSize * c_sizeof!(u8));
    bwt.deCode = c_malloc!(spaceSize * c_sizeof!(u8));
    bwt.sorted = c_malloc!(spaceSize * c_sizeof!(i32));
    if (bwt.block == NULL!()).as_bool() || (bwt.sorted == NULL!()).as_bool() || (bwt.deCode == NULL!()).as_bool() {
        BzpBwtDecodeFinish(bwt.cast());
        return NULL!();
    }
    bwt.nBlock = 0;
    bwt.oriPtr = 0;
    return bwt.cast();
}
