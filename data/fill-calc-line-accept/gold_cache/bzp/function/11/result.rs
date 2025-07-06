pub fn BzpDeHuffmanLen(mut inData: Ptr<InDeComdata>, mut huffman: Ptr<BzpHuffmanDecode>) -> i32 {
    let mut ch: u8;
    c_for!(let mut i: i32 = 0; i < huffman.nGroups; i += 1; {
        let mut val: i32 = BzpReadBits(BZP_BITS5!(), inData).cast();
        c_for!(let mut j: i32 = 0; j < huffman.alphaSize; j += 1; {
            ch = BzpReadBits(BZP_BIT!(), inData).cast();
            while ch != 0 {
                ch = BzpReadBits(BZP_BIT!(), inData).cast();
                val += if ch == 0 { 1 } else { -1 };
                ch = BzpReadBits(BZP_BIT!(), inData).cast();
            }
            if val < 1 || val > BZP_HUFFMAN_LEN_UPPER_LIMIT!() {
                return BZP_ERROR_DATA!();
            }
            huffman.len[i as usize][j as usize] = val;
        });
    });
    return BZP_OK!();
}