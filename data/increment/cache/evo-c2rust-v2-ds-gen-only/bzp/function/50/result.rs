pub fn BzpHuffmanDecodeStep(mut huffman: Ptr<BzpHuffmanDecode>, mut inData: Ptr<InDeComdata>) -> i32 {
    if (huffman.deCodeNum == BZP_ELEMS_NUM_IN_ONE_GROUP!()).as_bool() {
        huffman.deCodeNum = 0;
        huffman.selectCnt += 1;
    }
    let mut gid: i32 = huffman.select[huffman.selectCnt].cast();
    let mut chlen: i32 = huffman.minLens[gid].cast();
    let mut val: i32 = BzpReadBits(chlen.cast(), inData.cast()).cast();
    while (chlen < BZP_HUFFMAN_LEN_UPPER_LIMIT!()).as_bool() && (val > huffman.limit[gid][chlen]).as_bool() {
        chlen += 1;
        let mut nxtbit: i32 = BzpReadBits(1, inData.cast()).cast();
        val = (val << 1) | nxtbit;
    }
    if (chlen > BZP_HUFFMAN_LEN_UPPER_LIMIT!()).as_bool() {
        return -1;
    }
    val = (val - huffman.base[gid][chlen]).cast();
    val = huffman.perm[gid][val].cast();
    huffman.deCodeNum += 1;
    return val.cast();
}
