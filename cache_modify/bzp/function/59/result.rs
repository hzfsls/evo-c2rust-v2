pub fn BzpDeCompressOneBlock(mut inData: Ptr<InDeComdata>, mut huffman: Ptr<BzpHuffmanDecode>, mut debwt: Ptr<BzpBwtDecodeInfo>) -> i32 {
    let mut ret: i32 = BZP_OK!();
    BzpCheckFileHead(inData);
    let mut blockCRC: u32 = BzpReadUInt32(inData);
    BzpReadBits(BZP_BIT!(), inData).cast::<Void>();
    let mut oriPtr: i32 = BzpReadUInt24(inData).cast();
    if (oriPtr < 0 || oriPtr > BZP_BASE_BLOCK_SIZE!() * inData.blockSize) {
        return BZP_ERROR_DATA!();
    }
    let mut ninUse: i32 = BzpGetDictionaryList(inData);
    huffman.alphaSize = (ninUse + BZP_EXTRA_CHARS_NUM!());
    huffman.nGroups = BzpReadBits(BZP_BITS3!(), inData).cast::<i32>();
    if (huffman.nGroups < BZP_NGROUPS_NUM_0!() || huffman.nGroups > BZP_NGROUPS_NUM_4!()) {
        return BZP_ERROR_DATA!();
    }
    huffman.nSelect = BzpReadBits(BZP_BITS15!(), inData).cast();
    let mut nSelectUpperLimit: i32 = (inData.blockSize * BZP_BASE_BLOCK_SIZE!() / BZP_ELEMS_NUM_IN_ONE_GROUP!() + 1);
    if (huffman.nSelect < 1 || huffman.nSelect > nSelectUpperLimit) {
        return BZP_ERROR_DATA!();
    }
    ret |= BzpDeHuffmanSelect(inData, huffman);
    ret |= BzpDeHuffmanLen(inData, huffman);
    if (ret != BZP_OK!()) {
        return ret;
    }
    BzpGenerateDecodeTable(huffman);
    debwt.oriPtr = oriPtr;
    ret = BzpMTFDeCode(inData, huffman, debwt);
    if (ret != BZP_OK!() || (debwt.nBlock >= BZP_BASE_BLOCK_SIZE!() * inData.blockSize)) {
        return BZP_ERROR_DATA!();
    }
    BzpBwtDecode(debwt);
    ret = BzpDeCodeToStream(inData, debwt);
    if (ret != BZP_OK!()) {
        return ret;
    }
    inData.blockCRC = !(inData.blockCRC);
    if (blockCRC != inData.blockCRC) {
        ret = BZP_ERROR_DATA!();
    }
    return ret;
}
