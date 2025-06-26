pub fn BzpBlockSortInit(blockSize: i32) -> Ptr<BzpBwtInfo> {
    if BZP_INVALID_BLOCK_SIZE!(blockSize) {
        return NULL!();
    }
    let mut bwt: Ptr<BzpBwtInfo> = c_malloc!(c_sizeof!(BzpBwtInfo));
    if bwt == NULL!() {
        return NULL!();
    }
    c_memset_s!(bwt, c_sizeof!(BzpBwtInfo), 0, c_sizeof!(BzpBwtInfo));
    let spaceSize: i32 = blockSize * BZP_BASE_BLOCK_SIZE!();
    bwt.nBlockMax = spaceSize - BZP_BLOCK_RESERVED_SPACE_SIZE!();
    bwt.block = c_malloc!(spaceSize as u32 * c_sizeof!(u8));
    bwt.sortBlock = c_malloc!(spaceSize as u32 * c_sizeof!(i32));
    bwt.idx = c_malloc!(spaceSize as u32 * c_sizeof!(i32));
    bwt.isStartPos = c_malloc!(spaceSize as u32 * c_sizeof!(i32));
    if bwt.block == NULL!() || bwt.sortBlock == NULL!() || bwt.idx == NULL!() || bwt.isStartPos == NULL!() {
        BzpBwtFinish(bwt);
        return NULL!();
    }
    c_memset_s!(bwt.isStartPos, spaceSize as u32 * c_sizeof!(i32), 0, spaceSize as u32 * c_sizeof!(i32));
    bwt.blockCRC = BZP_INIT_BLOCK_CRC!();
    return bwt;
}