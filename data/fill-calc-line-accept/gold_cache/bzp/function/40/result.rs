pub fn BzpCompressOneBlock(mut bzpInfo: Ptr<BzpAlgorithmInfo>, mut outData: Ptr<BzpOutComdata>) -> i32 {
    let mut bwt: Ptr<BzpBwtInfo> = bzpInfo.bwt;
    let mut mtf: Ptr<BzpMtfInfo> = bzpInfo.mtf;
    let mut huffman: Ptr<BzpHuffmanGroups> = bzpInfo.huffman;
    let mut ret: i32 = BZP_OK!();
    if bwt.nBlock == 0 {
        return BZP_OK!();
    }
    BzpWriteFileHead(outData, bwt.blockId);
    if bwt.nBlock > 0 {
        BzpCalculateCRC(bwt);
        BzpBlockSortMain(bwt);
        BzpMtfReSet(mtf);
        mtf.block = bwt.block;
        mtf.map = bwt.sortBlock;
        mtf.inUse = bwt.inUse.cast();
        mtf.nBlock = bwt.nBlock;
        BzpMtfMain(mtf);
        ret = BzpHuffmanGroupsReset(huffman, mtf.nUse + BZP_EXTRA_CHARS_NUM!());
        if ret != BZP_OK!() {
            return ret;
        }
        huffman.block = mtf.mtfV;
        huffman.mtfFreq = mtf.mtfFreq.cast();
        huffman.nBlock = mtf.nMtf;
        BzpHuffmanMain(huffman);        
        BzpWriteBlockHead(outData, bwt);
        BzpWriteValidASCII(outData, bwt);
        BzpWriteToArray(huffman.nGroups, BZP_BITS3!(), outData);
        BzpWriteSelect(outData, huffman);
        BzpWriteLen(outData, huffman);
        BzpWriteInputEncode(outData, mtf, huffman);
    }
    return BZP_OK!();
}