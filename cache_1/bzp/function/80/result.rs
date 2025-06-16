pub fn BzpWriteLen(mut outData: Ptr<BzpOutComdata>, mut huffman: Ptr<BzpHuffmanGroups>) {
    c_for!(let mut i: i32 = 0; i < huffman.nGroups; i.suffix_plus_plus(); {
        let mut val: i32 = huffman.huffmanGroups[i].len[0];
        BzpWriteToArray(val.cast(), BZP_BITS5!(), outData.cast());

        c_for!(let mut j: i32 = 0; j < huffman.alphaSize; j.suffix_plus_plus(); {
            let mut tar: i32 = huffman.huffmanGroups[i].len[j];
            let mut deta: i32 = 0;
            let mut saveVal: i32 = 0;
            if (val < tar).as_bool() {
                saveVal = BZP_HUFFMAN_LEN_INCREASE!();
                deta = 1;
            } else if (val > tar).as_bool() {
                saveVal = BZP_HUFFMAN_LEN_REDUCED!();
                deta = -1;
            }
            while (val != tar).as_bool() {
                BzpWriteToArray(saveVal.cast(), BZP_BITS2!(), outData.cast());
                val += deta;
            }
            BzpWriteToArray(0, BZP_BIT!(), outData.cast());
        });
    });
}
