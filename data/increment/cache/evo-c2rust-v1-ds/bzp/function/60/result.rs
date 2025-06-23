pub fn BZPReadFileEnd(mut inData: Ptr<InDeComdata>, mut caltotalCRC: u32) -> i32 {
    let mut ch: u8 = BzpReadBits(BZP_BITS8!(), inData.cast()).cast();
    if ch != BZP_FILE_END_1!() {
        return BZP_ERROR_DATA!();
    }
    ch = BzpReadBits(BZP_BITS8!(), inData.cast()).cast();
    if ch != BZP_FILE_END_2!() {
        return BZP_ERROR_DATA!();
    }
    ch = BzpReadBits(BZP_BITS8!(), inData.cast()).cast();
    if ch != BZP_FILE_END_3!() {
        return BZP_ERROR_DATA!();
    }
    ch = BzpReadBits(BZP_BITS8!(), inData.cast()).cast();
    if ch != BZP_FILE_END_4!() {
        return BZP_ERROR_DATA!();
    }
    ch = BzpReadBits(BZP_BITS8!(), inData.cast()).cast();
    if ch != BZP_FILE_END_5!() {
        return BZP_ERROR_DATA!();
    }
    let mut storedcombinedcrc: u32 = BzpReadUInt32(inData.cast()).cast();
    if caltotalCRC != storedcombinedcrc {
        return BZP_ERROR_DATA!();
    }
    return BZP_OK!();
}
