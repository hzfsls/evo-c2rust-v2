pub fn BzpReadFileHead(mut inData: Ptr<InDeComdata>) -> i32 {
    let mut ch: u8 = BzpReadBits(BZP_BITS8!(), inData.cast()).cast();
    if (ch != BZP_HDR_B!()).as_bool() {
        return BZP_ERROR_DATA_MAGIC!();
    }
    ch = BzpReadBits(BZP_BITS8!(), inData.cast()).cast();
    if (ch != BZP_HDR_Z!()).as_bool() {
        return BZP_ERROR_DATA_MAGIC!();
    }
    ch = BzpReadBits(BZP_BITS8!(), inData.cast()).cast();
    if (ch != BZP_HDR_H!()).as_bool() {
        return BZP_ERROR_DATA_MAGIC!();
    }
    ch = BzpReadBits(BZP_BITS8!(), inData.cast()).cast();
    let mut blockSize: i32 = (ch - BZP_HDR_0!()).cast();
    if BZP_INVALID_BLOCK_SIZE!(blockSize).as_bool() {
        return BZP_ERROR_DATA_MAGIC!();
    }
    inData.blockSize = blockSize.cast();
    return BZP_OK!();
}
