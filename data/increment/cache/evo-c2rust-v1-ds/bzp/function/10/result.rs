pub fn BzpHuffmanGroupsReset(mut huffman: Ptr<BzpHuffmanGroups>, mut alphaSize: i32) -> i32 {
    if BZP_INVALID_ALPHA_SIZE!(alphaSize) {
        return BZP_ERROR_PARAM!();
    }

    huffman.alphaSize = alphaSize;
    huffman.block = NULL!();
    huffman.mtfFreq = NULL!();
    huffman.nSelect = 0;
    huffman.nGroups = 0;

    c_for!(let mut i: i32 = 0; i < BZP_MAX_GROUPS_NUM!(); i.suffix_plus_plus(); {
        BzpHuffmanInit(alphaSize.cast(), c_ref!(huffman.huffmanGroups[i]).cast());
    });
    return BZP_OK!();
}
