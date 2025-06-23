pub unsafe extern "C" fn BzpAlgorithmInfoFinish(bzpInfo: *mut BzpAlgorithmInfo) {
    if !bzpInfo.is_null() {
        BzpBwtFinish((*bzpInfo).bwt);
        BzpMtfFinish((*bzpInfo).mtf);
        BzpHuffmanGroupsFinish((*bzpInfo).huffman);
        BzpFileFinish((*bzpInfo).compressFile);
        BzpOutComDataFinish((*bzpInfo).outData);
        libc::free(bzpInfo as *mut libc::c_void);
    }
}
