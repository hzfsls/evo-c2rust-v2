pub fn BzpCheckFileHead(mut inData: Ptr<InDeComdata>) -> i32 {
    let mut ch: u8 = Default::default();
    ch = BzpReadBits(BZP_BITS8!(), inData.cast()).cast();
    if ch != BZP_BLOCK_HEAD_1!() {
        return BZP_ERROR_DATA!();
    }
    ch = BzpReadBits(BZP_BITS8!(), inData.cast()).cast();
    if ch != BZP_BLOCK_HEAD_2!() {
        return BZP_ERROR_DATA!();
    }
    ch = BzpReadBits(BZP_BITS8!(), inData.cast()).cast();
    if ch != BZP_BLOCK_HEAD_3!() {
        return BZP_ERROR_DATA!();
    }
    ch = BzpReadBits(BZP_BITS8!(), inData.cast()).cast();
    if ch != BZP_BLOCK_HEAD_4!() {
        return BZP_ERROR_DATA!();
    }
    ch = BzpReadBits(BZP_BITS8!(), inData.cast()).cast();
    if ch != BZP_BLOCK_HEAD_5!() {
        return BZP_ERROR_DATA!();
    }
    return BZP_OK!();
}
