pub fn BzpReadUInt32(mut inData: Ptr<InDeComdata>) -> u32 {
    let mut ch: u8 = Default::default();
    let mut val: u32 = 0;
    ch = BzpReadBits(BZP_BITS8!(), inData.cast()).cast();
    val = (val << BZP_BITS8!()) | ch.cast::<u32>();
    ch = BzpReadBits(BZP_BITS8!(), inData.cast()).cast();
    val = (val << BZP_BITS8!()) | ch.cast::<u32>();
    ch = BzpReadBits(BZP_BITS8!(), inData.cast()).cast();
    val = (val << BZP_BITS8!()) | ch.cast::<u32>();
    ch = BzpReadBits(BZP_BITS8!(), inData.cast()).cast();
    val = (val << BZP_BITS8!()) | ch.cast::<u32>();
    return val;
}
