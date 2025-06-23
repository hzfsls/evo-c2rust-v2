pub fn BZPDeCompressData(mut inData: Ptr<InDeComdata>) -> i32 {
    let mut ret: i32 = BZP_OK!();
    let mut caltotalCRC: u32 = 0;
    let mut ch: u8 = Default::default();
    ret = BzpReadFileHead(inData.cast()).cast();
    if (ret != BZP_OK!()).as_bool() {
        return ret.cast();
    }
    let mut huffman: Ptr<BzpHuffmanDecode> = BzpHuffmanDecodeInit(inData.blockSize.cast()).cast();
    let mut debwt: Ptr<BzpBwtDecodeInfo> = BzpBwtDecodeInit(inData.blockSize.cast()).cast();
    loop {
        ch = BzpReadBits(BZP_BITS8!(), inData.cast()).cast();
        if (ch != BZP_FILE_END_0!()).as_bool() {
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
            caltotalCRC = (caltotalCRC << 1) | (caltotalCRC >> BZP_CRC_MOVE_RIGHT_VAL!()).cast();
            caltotalCRC ^= inData.blockCRC.cast();
        } else {
            break;
        }
    }
    if (ret == BZP_OK!()).as_bool() {
        ret = BZPReadFileEnd(inData.cast(), caltotalCRC.cast()).cast();
    }
    BzpHuffmanDecodeFinish(huffman.cast());
    BzpBwtDecodeFinish(debwt.cast());
    return ret.cast();
}