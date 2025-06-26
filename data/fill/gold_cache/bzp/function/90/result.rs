pub fn BzpGenerateDecodeTable(mut huffman: Ptr<BzpHuffmanDecode>) {
    c_for!(let mut t: i32 = 0; t < huffman.nGroups; t += 1; {
        BzpGetOneTable(huffman, t);
    });
}