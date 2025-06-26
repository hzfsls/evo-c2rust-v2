pub fn BzpGenerateDecodeTable(mut huffman: Ptr<BzpHuffmanDecode>) {
    c_for!(let mut t: i32 = 0; t < huffman.nGroups; t.suffix_plus_plus(); {
        BzpGetOneTable(huffman.cast(), t.cast());
    });
}
