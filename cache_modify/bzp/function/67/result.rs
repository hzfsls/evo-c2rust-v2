pub fn BzpAlgorithmInfoFinish(mut bzpInfo: Ptr<BzpAlgorithmInfo>) {
    if (bzpInfo != NULL!()).as_bool() {
        BzpBwtFinish(bzpInfo.bwt.cast());
        BzpMtfFinish(bzpInfo.mtf.cast());
        BzpBzpHuffmanGroupsFinish(bzpInfo.huffman.cast());
        BzpFileFinish(bzpInfo.compressFile.cast());
        BzpOutComDataFinish(bzpInfo.outData.cast());
        c_free!(bzpInfo);
    }
}
