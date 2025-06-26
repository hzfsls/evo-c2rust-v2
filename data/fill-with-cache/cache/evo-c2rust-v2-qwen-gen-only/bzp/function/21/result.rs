pub fn BzpAlgorithmInfoInit(mut blockSize: i32) -> Ptr<BzpAlgorithmInfo> {
    let mut bzpInfo: Ptr<BzpAlgorithmInfo> = c_malloc!(c_sizeof!(BzpAlgorithmInfo));
    if (bzpInfo == NULL!()).as_bool() {
        return NULL!();
    }
    bzpInfo.bwt = BzpBlockSortInit(blockSize.cast()).cast();
    bzpInfo.mtf = BzpMtfInit(blockSize.cast()).cast();
    bzpInfo.huffman = BzpHuffmanGroupsInit(blockSize.cast()).cast();
    bzpInfo.outData = BzpOutComDataInit(blockSize.cast()).cast();
    bzpInfo.compressFile = BzpFileInit().cast();
    if (bzpInfo.bwt == NULL!()).as_bool() || (bzpInfo.outData == NULL!()).as_bool() || (bzpInfo.compressFile == NULL!()).as_bool() || (bzpInfo.mtf == NULL!()).as_bool() || (bzpInfo.huffman == NULL!()).as_bool() {
        BzpAlgorithmInfoFinish(bzpInfo.cast());
        return NULL!();
    }
    return bzpInfo.cast();
}