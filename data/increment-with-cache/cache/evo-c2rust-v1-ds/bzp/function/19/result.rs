pub fn BzpBlockSortInit(mut blockSize: i32) -> Ptr<BzpBwtInfo> {
    if BZP_INVALID_BLOCK_SIZE!(blockSize) {
        return NULL!();
    }
    let mut bwt: Ptr<BzpBwtInfo> = c_malloc!(c_sizeof!(BzpBwtInfo));
    if bwt == NULL!() {
        return NULL!();
    }

    c_memset_s!(bwt, c_sizeof!(BzpBwtInfo), 0, c_sizeof!(BzpBwtInfo)).cast::<Void>();

    let mut spaceSize: i32 = blockSize * BZP_BASE_BLOCK_SIZE!();
    bwt.nBlockMax = (spaceSize - BZP_BLOCK_RESERVED_SPACE_SIZE!()).cast();
    bwt.block = c_malloc!(spaceSize * c_sizeof!(u8));
    bwt.sortBlock = c_malloc!(spaceSize * c_sizeof!(i32));
    bwt.idx = c_malloc!(spaceSize * c_sizeof!(i32));
    bwt.isStartPos = c_malloc!(spaceSize * c_sizeof!(i32));
    if bwt.block == NULL!() || bwt.sortBlock == NULL!() || bwt.idx == NULL!() || bwt.isStartPos == NULL!() {
        BzpBwtFinish(bwt.cast());
        return NULL!();
    }

    c_memset_s!(bwt.isStartPos, spaceSize * c_sizeof!(i32), 0, spaceSize * c_sizeof!(i32)).cast::<Void>();
    bwt.blockCRC = BZP_INIT_BLOCK_CRC!();
    return bwt.cast();
}
