pub fn BzpAlgorithmInfoInit(mut blockSize: i32) -> Ptr<BzpAlgorithmInfo> {
    let mut bzpInfo: Ptr<BzpAlgorithmInfo> = c_malloc!(c_sizeof!(BzpAlgorithmInfo));
    if bzpInfo == NULL!() {
        return NULL!();
    }
    bzpInfo.bwt = BzpBlockSortInit(blockSize.cast());
    bzpInfo.mtf = BzpMtfInit(blockSize.cast());
    bzpInfo.huffman = BzpHuffmanGroupsInit(blockSize.cast());
    bzpInfo.outData = BzpOutComDataInit(blockSize.cast());
    bzpInfo.compressFile = BzpFileInit();
    if bzpInfo.bwt == NULL!() || bzpInfo.outData == NULL!() || bzpInfo.compressFile == NULL!() || bzpInfo.mtf == NULL!() || bzpInfo.huffman == NULL!() {
        BzpAlgorithmInfoFinish(bzpInfo.cast());
        return NULL!();
    }
    return bzpInfo.cast();
}
