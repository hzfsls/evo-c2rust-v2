pub fn BzpReadFileHead(mut inData: Ptr<InDeComdata>) -> i32 {
    let mut ch: u8 = BzpReadBits(BZP_BITS8!(), inData.cast()).cast();
    if ch != BZP_HDR_B!() {
        return BZP_ERROR_DATA_MAGIC!();
    }
    ch = BzpReadBits(BZP_BITS8!(), inData.cast()).cast();
    if ch != BZP_HDR_Z!() {
        return BZP_ERROR_DATA_MAGIC!();
    }
    ch = BzpReadBits(BZP_BITS8!(), inData.cast()).cast();
    if ch != BZP_HDR_H!() {
        return BZP_ERROR_DATA_MAGIC!();
    }
    ch = BzpReadBits(BZP_BITS8!(), inData.cast()).cast();
    let mut blockSize: i32 = (ch - BZP_HDR_0!()).cast();
    if BZP_INVALID_BLOCK_SIZE!(blockSize) {
        return BZP_ERROR_DATA_MAGIC!();
    }
    inData.blockSize = blockSize.cast();
    return BZP_OK!();
}
