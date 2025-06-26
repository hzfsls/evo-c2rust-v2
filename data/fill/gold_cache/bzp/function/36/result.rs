pub fn BzpWriteLen(mut outData: Ptr<BzpOutComdata>, mut huffman: Ptr<BzpHuffmanGroups>) {
    c_for!(let mut i: i32 = 0; i < huffman.nGroups; i += 1; {
        let mut val: i32 = huffman.huffmanGroups[i].len[0];
        BzpWriteToArray(val, BZP_BITS5!(), outData);
        c_for!(let mut j: i32 = 0; j < huffman.alphaSize; j += 1; {
            let mut tar: i32 = huffman.huffmanGroups[i].len[j];
            let mut deta: i32 = 0;
            let mut saveVal: i32 = 0;
            if val < tar {
                saveVal = BZP_HUFFMAN_LEN_INCREASE!();
                deta = 1;
            } else if val > tar {
                saveVal = BZP_HUFFMAN_LEN_REDUCED!();
                deta = -1;
            }
            while val != tar {
                BzpWriteToArray(saveVal, BZP_BITS2!(), outData);
                val += deta;
            }
            BzpWriteToArray(0, BZP_BIT!(), outData);
        });
    });
}