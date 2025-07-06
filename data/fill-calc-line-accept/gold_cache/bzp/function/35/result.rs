pub fn BzpWriteSelect(mut outData: Ptr<BzpOutComdata>, mut huffman: Ptr<BzpHuffmanGroups>) {
    BzpWriteToArray(huffman.nSelect, BZP_BITS15!(), outData);
    c_for!(let mut i: i32 = 0; i < huffman.nSelect; i += 1; {
        c_for!(let mut j: i32 = 0; j < huffman.selectMTF[i]; j += 1; {
            BzpWriteToArray(1, BZP_BIT!(), outData);
        });
        BzpWriteToArray(0, BZP_BIT!(), outData);
    });
}