pub fn BzpAlgorithmInfoFinish(mut bzpInfo: Ptr<BzpAlgorithmInfo>) {
    if bzpInfo != NULL!() {
        BzpBwtFinish(bzpInfo.bwt);
        BzpMtfFinish(bzpInfo.mtf);
        BzpHuffmanGroupsFinish(bzpInfo.huffman);
        BzpFileFinish(bzpInfo.compressFile);
        BzpOutComDataFinish(bzpInfo.outData);
        c_free!(bzpInfo);
    }
}