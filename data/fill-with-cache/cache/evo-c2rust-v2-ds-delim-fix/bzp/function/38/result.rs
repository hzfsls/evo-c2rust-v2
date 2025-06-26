pub fn BzpWriteFileEnd(mut outData: Ptr<BzpOutComdata>, mut combinedCRC: i32) {
    BzpWriteToArray(BZP_FILE_END_0!(), BZP_BITS8!(), outData.cast());
    BzpWriteToArray(BZP_FILE_END_1!(), BZP_BITS8!(), outData.cast());
    BzpWriteToArray(BZP_FILE_END_2!(), BZP_BITS8!(), outData.cast());
    BzpWriteToArray(BZP_FILE_END_3!(), BZP_BITS8!(), outData.cast());
    BzpWriteToArray(BZP_FILE_END_4!(), BZP_BITS8!(), outData.cast());
    BzpWriteToArray(BZP_FILE_END_5!(), BZP_BITS8!(), outData.cast());
    BzpWriteInt32(combinedCRC.cast(), outData.cast());
}
