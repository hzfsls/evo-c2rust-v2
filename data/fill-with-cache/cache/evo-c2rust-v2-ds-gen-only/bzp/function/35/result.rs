pub fn BzpWriteSelect(mut outData: Ptr<BzpOutComdata>, mut huffman: Ptr<BzpHuffmanGroups>) {
    BzpWriteToArray(huffman.nSelect.cast(), BZP_BITS15!(), outData.cast());
    c_for!(let mut i: i32 = 0; i < huffman.nSelect; i.suffix_plus_plus(); {
        c_for!(let mut j: i32 = 0; j < huffman.selectMTF[i]; j.suffix_plus_plus(); {
            BzpWriteToArray(1, BZP_BIT!(), outData.cast());
        });
        BzpWriteToArray(0, BZP_BIT!(), outData.cast());
    });
}
