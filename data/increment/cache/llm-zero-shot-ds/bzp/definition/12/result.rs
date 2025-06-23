pub struct BzpAlgorithmInfo {
    pub bwt: *mut BzpBwtInfo,
    pub huffman: *mut BzpHuffmanGroups,
    pub mtf: *mut BzpMtfInfo,
    pub compressFile: *mut BzpFile,
    pub outData: *mut BzpOutComdata,
}
