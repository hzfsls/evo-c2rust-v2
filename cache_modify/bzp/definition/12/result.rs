#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct BzpAlgorithmInfo {
    pub bwt: Ptr<BzpBwtInfo>,
    pub huffman: Ptr<BzpHuffmanGroups>,
    pub mtf: Ptr<BzpMtfInfo>,
    pub compressFile: Ptr<BzpFile>,
    pub outData: Ptr<BzpOutComdata>,
}
