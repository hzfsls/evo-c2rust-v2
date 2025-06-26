pub fn BzpDeHuffmanLen(mut inData: Ptr<InDeComdata>, mut huffman: Ptr<BzpHuffmanDecode>) -> i32 {
    let mut ch: u8 = Default::default();
    c_for!(let mut i: i32 = 0; i < huffman.nGroups; i.suffix_plus_plus(); {
        let mut val: i32 = BzpReadBits(BZP_BITS5!(), inData.cast()).cast();
        c_for!(let mut j: i32 = 0; j < huffman.alphaSize; j.suffix_plus_plus(); {
            ch = BzpReadBits(BZP_BIT!(), inData.cast()).cast();
            while ch != 0 {
                ch = BzpReadBits(BZP_BIT!(), inData.cast()).cast();
                val += if ch == 0 { 1 } else { -1 };
                ch = BzpReadBits(BZP_BIT!(), inData.cast()).cast();
            }
            if val < 1 || val > BZP_HUFFMAN_LEN_UPPER_LIMIT!() {
                return BZP_ERROR_DATA!();
            }
            huffman.len[i][j] = val.cast();
        });
    });
    return BZP_OK!();
}
