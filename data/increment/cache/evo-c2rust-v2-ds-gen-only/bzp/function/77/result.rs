pub fn BzpWriteBlockHead(mut outData: Ptr<BzpOutComdata>, mut bwt: Ptr<BzpBwtInfo>) {
    BzpWriteToArray(BZP_BLOCK_HEAD_0!(), BZP_BITS8!(), outData.cast());
    BzpWriteToArray(BZP_BLOCK_HEAD_1!(), BZP_BITS8!(), outData.cast());
    BzpWriteToArray(BZP_BLOCK_HEAD_2!(), BZP_BITS8!(), outData.cast());
    BzpWriteToArray(BZP_BLOCK_HEAD_3!(), BZP_BITS8!(), outData.cast());
    BzpWriteToArray(BZP_BLOCK_HEAD_4!(), BZP_BITS8!(), outData.cast());
    BzpWriteToArray(BZP_BLOCK_HEAD_5!(), BZP_BITS8!(), outData.cast());
    BzpWriteInt32(bwt.blockCRC.cast(), outData.cast());
    BzpWriteToArray(0, BZP_BIT!(), outData.cast());
    BzpWriteToArray(bwt.oriPtr.cast(), BZP_BITS24!(), outData.cast());
}
