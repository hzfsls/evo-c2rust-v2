pub struct BzpAlgorithmInfo {
    pub bwt: *mut BzpBwtInfo,
    pub huffman: *mut BzpHuffmanGroups,
    pub mtf: *mut BzpMtfInfo,
    pub compress_file: *mut BzpFile,
    pub out_data: *mut BzpOutComdata,
}
