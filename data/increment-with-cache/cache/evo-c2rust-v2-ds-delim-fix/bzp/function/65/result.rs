pub fn BzpAlgorithmInfoInit(mut blockSize: i32) -> Ptr<BzpAlgorithmInfo> {
    let mut bzpInfo: Ptr<BzpAlgorithmInfo> = c_malloc!(c_sizeof!(BzpAlgorithmInfo));
    if (bzpInfo == NULL!()).as_bool() {
        return NULL!();
    }
    bzpInfo.bwt = BzpBlockSortInit(blockSize.cast());
    bzpInfo.mtf = BzpMtfInit(blockSize.cast());
    bzpInfo.huffman = BzpHuffmanGroupsInit(blockSize.cast());
    bzpInfo.outData = BzpOutComDataInit(blockSize.cast());
    bzpInfo.compressFile = BzpFileInit();
    if (bzpInfo.bwt == NULL!()).as_bool() || (bzpInfo.outData == NULL!()).as_bool() || (bzpInfo.compressFile == NULL!()).as_bool() || (bzpInfo.mtf == NULL!()).as_bool() || (bzpInfo.huffman == NULL!()).as_bool() {
        BzpAlgorithmInfoFinish(bzpInfo.cast());
        return NULL!();
    }
    return bzpInfo.cast();
}
