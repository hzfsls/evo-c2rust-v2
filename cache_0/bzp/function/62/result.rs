pub fn BZPDeCompressData(mut inData: Ptr<InDeComdata>) -> i32 {
    let mut ret: i32 = BZP_OK!();
    let mut caltotalCRC: u32 = 0;
    let mut ch: u8;
    ret = BzpReadFileHead(inData.cast()).cast();
    if (ret != BZP_OK!()).as_bool() {
        return ret;
    }
    let mut huffman: Ptr<BzpHuffmanDecode> = BzpHuffmanDecodeInit(inData.blockSize.cast());
    let mut debwt: Ptr<BzpBwtDecodeInfo> = BzpBwtDecodeInit(inData.blockSize.cast());

    while {
        ch = BzpReadBits(BZP_BITS8!(), inData.cast()).cast();
        ch != BZP_FILE_END_0!()
    } {
        if (ch != BZP_BLOCK_HEAD_0!()).as_bool() {
            ret = BZP_ERROR_DATA!();
            break;
        }
        BzpHuffmanDecodeReset(huffman.cast());
        inData.blockCRC = BZP_INIT_BLOCK_CRC!();

        ret = BzpDeCompressOneBlock(inData.cast(), huffman.cast(), debwt.cast()).cast();
        if (ret != BZP_OK!()).as_bool() {
            break;
        }

        caltotalCRC = (caltotalCRC << 1) | (caltotalCRC >> BZP_CRC_MOVE_RIGHT_VAL!());
        caltotalCRC ^= inData.blockCRC.cast();
    }
    if (ret == BZP_OK!()).as_bool() {
        ret = BZPReadFileEnd(inData.cast(), caltotalCRC.cast()).cast();
    }
    BzpHuffmanDecodeFinish(huffman.cast());
    BzpBwtDecodeFinish(debwt.cast());
    return ret.cast();
}
