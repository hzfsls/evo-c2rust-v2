pub fn BzpWriteSelect(mut outData: Ptr<BzpOutComdata>, mut huffman: Ptr<BzpHuffmanGroups>) {
    BzpWriteToArray(huffman.nSelect.cast(), BZP_BITS15!().cast(), outData.cast());
    c_for!(let mut i: i32 = 0; i < huffman.nSelect; i.suffix_plus_plus(); {
        c_for!(let mut j: i32 = 0; j < huffman.selectMTF[i]; j.suffix_plus_plus(); {
            BzpWriteToArray(1.cast(), BZP_BIT!().cast(), outData.cast());
        });
        BzpWriteToArray(0.cast(), BZP_BIT!().cast(), outData.cast());
    });
}