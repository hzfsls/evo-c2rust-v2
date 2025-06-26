pub fn BzpBwtDecodeInit(blockSize: i32) -> Ptr<BzpBwtDecodeInfo> {
    if BZP_INVALID_BLOCK_SIZE!(blockSize) {
        return NULL!();
    }
    let mut bwt: Ptr<BzpBwtDecodeInfo> = c_malloc!(c_sizeof!(BzpBwtDecodeInfo));
    if bwt == NULL!() {
        return NULL!();
    }
    let spaceSize: i32 = BZP_BASE_BLOCK_SIZE!() * blockSize;
    bwt.block = c_malloc!(spaceSize as u32 * c_sizeof!(u8));
    bwt.deCode = c_malloc!(spaceSize as u32 * c_sizeof!(u8));
    bwt.sorted = c_malloc!(spaceSize as u32 * c_sizeof!(i32));
    if bwt.block == NULL!() || bwt.sorted == NULL!() || bwt.deCode == NULL!() {
        BzpBwtDecodeFinish(bwt);
        return NULL!();
    }
    bwt.nBlock = 0;
    bwt.oriPtr = 0;
    return bwt;
}