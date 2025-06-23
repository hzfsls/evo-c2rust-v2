pub fn BzpHuffmanDecodeStep(mut huffman: Ptr<BzpHuffmanDecode>, mut inData: Ptr<InDeComdata>) -> i32 {
    if (huffman.deCodeNum == BZP_ELEMS_NUM_IN_ONE_GROUP!()) {
        huffman.deCodeNum = 0;
        huffman.selectCnt += 1;
    }
    let tmp0 = huffman.selectCnt;
    let mut gid: i32 = huffman.select[tmp0];
    let mut chlen: i32 = huffman.minLens[gid];
    let mut val: i32 = BzpReadBits(chlen, inData).cast();
    while (chlen < BZP_HUFFMAN_LEN_UPPER_LIMIT!()) && (val > huffman.limit[gid][chlen]) {
        chlen += 1;
        let mut nxtbit: i32 = BzpReadBits(1, inData).cast();
        val = (val << 1) | nxtbit;
    }
    if (chlen > BZP_HUFFMAN_LEN_UPPER_LIMIT!()) {
        return -1;
    }
    val = (val - huffman.base[gid][chlen]);
    val = huffman.perm[gid][val];
    huffman.deCodeNum += 1;
    return val;
}
