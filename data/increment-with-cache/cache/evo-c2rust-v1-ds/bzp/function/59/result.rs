pub fn BzpDeCompressOneBlock(mut inData: Ptr<InDeComdata>, mut huffman: Ptr<BzpHuffmanDecode>, mut debwt: Ptr<BzpBwtDecodeInfo>) -> i32 {
    let mut ret: i32 = BZP_OK!();
    BzpCheckFileHead(inData.cast());
    let mut blockCRC: u32 = BzpReadUInt32(inData.cast());
    BzpReadBits(BZP_BIT!(), inData.cast()).cast::<Void>();
    let mut oriPtr: i32 = BzpReadUInt24(inData.cast()).cast();
    if oriPtr < 0 || oriPtr > BZP_BASE_BLOCK_SIZE!() * inData.blockSize {
        return BZP_ERROR_DATA!();
    }
    let mut ninUse: i32 = BzpGetDictionaryList(inData.cast()).cast();
    huffman.alphaSize = (ninUse + BZP_EXTRA_CHARS_NUM!()).cast();
    huffman.nGroups = BzpReadBits(BZP_BITS3!(), inData.cast()).cast();
    if huffman.nGroups < BZP_NGROUPS_NUM_0!() || huffman.nGroups > BZP_NGROUPS_NUM_4!() {
        return BZP_ERROR_DATA!();
    }
    huffman.nSelect = BzpReadBits(BZP_BITS15!(), inData.cast()).cast();
    let mut nSelectUpperLimit: i32 = (inData.blockSize * BZP_BASE_BLOCK_SIZE!() / BZP_ELEMS_NUM_IN_ONE_GROUP!() + 1).cast();
    if huffman.nSelect < 1 || huffman.nSelect > nSelectUpperLimit {
        return BZP_ERROR_DATA!();
    }
    ret |= BzpDeHuffmanSelect(inData.cast(), huffman.cast()).cast();
    ret |= BzpDeHuffmanLen(inData.cast(), huffman.cast()).cast();
    if ret != BZP_OK!() {
        return ret;
    }
    BzpGenerateDecodeTable(huffman.cast());
    debwt.oriPtr = oriPtr.cast();
    ret = BzpMTFDeCode(inData.cast(), huffman.cast(), debwt.cast()).cast();
    if ret != BZP_OK!() || debwt.nBlock >= BZP_BASE_BLOCK_SIZE!() * inData.blockSize {
        return BZP_ERROR_DATA!();
    }
    BzpBwtDecode(debwt.cast());
    ret = BzpDeCodeToStream(inData.cast(), debwt.cast()).cast();
    if ret != BZP_OK!() {
        return ret;
    }
    inData.blockCRC = !(inData.blockCRC);
    if blockCRC != inData.blockCRC {
        ret = BZP_ERROR_DATA!();
    }
    return ret.cast();
}
