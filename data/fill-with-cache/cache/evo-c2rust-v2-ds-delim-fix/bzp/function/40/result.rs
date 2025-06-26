pub fn BzpCompressOneBlock(mut bzpInfo: Ptr<BzpAlgorithmInfo>, mut outData: Ptr<BzpOutComdata>) -> i32 {
    let mut bwt: Ptr<BzpBwtInfo> = bzpInfo.bwt.cast();
    let mut mtf: Ptr<BzpMtfInfo> = bzpInfo.mtf.cast();
    let mut huffman: Ptr<BzpHuffmanGroups> = bzpInfo.huffman.cast();
    let mut ret: i32 = BZP_OK!();
    if (bwt.nBlock == 0).as_bool() {
        return BZP_OK!();
    }
    BzpWriteFileHead(outData.cast(), bwt.blockId.cast());
    if (bwt.nBlock > 0).as_bool() {
        BzpCalculateCRC(bwt.cast());
        BzpBlockSortMain(bwt.cast());
        BzpMtfReSet(mtf.cast());
        mtf.block = bwt.block.cast();
        mtf.map = bwt.sortBlock.cast();
        mtf.inUse = bwt.inUse.cast();
        mtf.nBlock = bwt.nBlock.cast();
        BzpMtfMain(mtf.cast());
        ret = BzpHuffmanGroupsReset(huffman.cast(), (mtf.nUse + BZP_EXTRA_CHARS_NUM!()).cast()).cast();
        if (ret != BZP_OK!()).as_bool() {
            return ret;
        }
        huffman.block = mtf.mtfV.cast();
        huffman.mtfFreq = mtf.mtfFreq.cast();
        huffman.nBlock = mtf.nMtf.cast();
        BzpHuffmanMain(huffman.cast());
        BzpWriteBlockHead(outData.cast(), bwt.cast());
        BzpWriteValidASCII(outData.cast(), bwt.cast());
        BzpWriteToArray(huffman.nGroups.cast(), BZP_BITS3!(), outData.cast());
        BzpWriteSelect(outData.cast(), huffman.cast());
        BzpWriteLen(outData.cast(), huffman.cast());
        BzpWriteInputEncode(outData.cast(), mtf.cast(), huffman.cast());
    }
    return BZP_OK!();
}
