pub fn BzpReadUInt24(mut inData: Ptr<InDeComdata>) -> u32 {
    let mut ch: u8;
    let mut val: u32 = 0;
    ch = BzpReadBits(BZP_BITS8!(), inData).cast();
    val = ((val << BZP_BITS8!()) | ch as u32).cast();
    ch = BzpReadBits(BZP_BITS8!(), inData).cast();
    val = ((val << BZP_BITS8!()) | ch as u32).cast();
    ch = BzpReadBits(BZP_BITS8!(), inData).cast();
    val = ((val << BZP_BITS8!()) | ch as u32).cast();
    return val;
}