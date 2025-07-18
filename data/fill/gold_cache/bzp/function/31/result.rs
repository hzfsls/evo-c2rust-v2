pub fn BzpWriteFileHead(mut outData: Ptr<BzpOutComdata>, mut blockId: i32) {
    if blockId == 0 {
        BzpWriteToArray(BZP_HDR_B!(), BZP_BITS8!(), outData);
        BzpWriteToArray(BZP_HDR_Z!(), BZP_BITS8!(), outData);
        BzpWriteToArray(BZP_HDR_H!(), BZP_BITS8!(), outData);
        BzpWriteToArray(BZP_HDR_0!() + outData.blockSize, BZP_BITS8!(), outData);
    }
}