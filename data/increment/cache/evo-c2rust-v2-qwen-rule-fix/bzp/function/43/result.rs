pub fn BZPDeCompressData(mut inData: Ptr<InDeComdata>) -> i32 {
    let mut ret: i32 = BZP_OK!();
    let mut caltotalCRC: u32 = 0;
    let mut ch: u8 = Default::default();
    ret = BzpReadFileHead(inData);
    if (ret != BZP_OK!()) {
        return ret;
    }
    let mut huffman: Ptr<BzpHuffmanDecode> = BzpHuffmanDecodeInit(inData.blockSize);
    let mut debwt: Ptr<BzpBwtDecodeInfo> = BzpBwtDecodeInit(inData.blockSize);
    loop {
        ch = BzpReadBits(BZP_BITS8!(), inData);
        if (ch != BZP_FILE_END_0!()) {
            if (ch != BZP_BLOCK_HEAD_0!()) {
                ret = BZP_ERROR_DATA!();
                break;
            }
            BzpHuffmanDecodeReset(huffman);
            inData.blockCRC = BZP_INIT_BLOCK_CRC!();
            ret = BzpDeCompressOneBlock(inData, huffman, debwt);
            if (ret != BZP_OK!()) {
                break;
            }
            caltotalCRC = (caltotalCRC << 1) | (caltotalCRC >> BZP_CRC_MOVE_RIGHT_VAL!());
            caltotalCRC ^= inData.blockCRC;
        } else {
            break;
        }
    }
    if (ret == BZP_OK!()) {
        ret = BZPReadFileEnd(inData, caltotalCRC);
    }
    BzpHuffmanDecodeFinish(huffman);
    BzpBwtDecodeFinish(debwt);
    return ret;
}