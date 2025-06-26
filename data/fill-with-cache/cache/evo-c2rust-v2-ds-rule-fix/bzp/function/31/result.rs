pub fn BzpWriteFileHead(mut outData: Ptr<BzpOutComdata>, mut blockId: i32) {
    if (blockId == 0).as_bool() {
        BzpWriteToArray(BZP_HDR_B!(), BZP_BITS8!(), outData.cast());
        BzpWriteToArray(BZP_HDR_Z!(), BZP_BITS8!(), outData.cast());
        BzpWriteToArray(BZP_HDR_H!(), BZP_BITS8!(), outData.cast());
        BzpWriteToArray((BZP_HDR_0!() + outData.blockSize).cast(), BZP_BITS8!(), outData.cast());
    }
}
