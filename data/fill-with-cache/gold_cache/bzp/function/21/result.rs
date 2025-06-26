pub fn BzpAlgorithmInfoInit(blockSize: i32) -> Ptr<BzpAlgorithmInfo> {
    let mut bzpInfo: Ptr<BzpAlgorithmInfo> = c_malloc!(c_sizeof!(BzpAlgorithmInfo));
    if bzpInfo == NULL!() {
        return NULL!();
    }
    bzpInfo.bwt = BzpBlockSortInit(blockSize);
    bzpInfo.mtf = BzpMtfInit(blockSize);
    bzpInfo.huffman = BzpHuffmanGroupsInit(blockSize);
    bzpInfo.outData = BzpOutComDataInit(blockSize);
    bzpInfo.compressFile = BzpFileInit();
    if bzpInfo.bwt == NULL!() || bzpInfo.outData == NULL!() || bzpInfo.compressFile == NULL!() || bzpInfo.mtf == NULL!() || bzpInfo.huffman == NULL!() {
        BzpAlgorithmInfoFinish(bzpInfo);
        return NULL!();
    }
    return bzpInfo;
}