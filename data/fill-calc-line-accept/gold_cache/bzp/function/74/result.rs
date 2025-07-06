pub fn BzpGetHuffmanTable(mut huffman: Ptr<BzpHuffmanInfo>) {
    let mut vec: i32 = 0;
    let mut mi: i32 = huffman.len[0];
    let mut mx: i32 = huffman.len[0];
    c_for!(let mut i = 0; i < huffman.alphaSize; i += 1; {
        mi = BZP_MIN_FUN!(mi, huffman.len[i]);
        mx = BZP_MAX_FUN!(mx, huffman.len[i]);
    });
    c_for!(let mut i = mi; i <= mx; i += 1; {
        c_for!(let mut j = 0; j < huffman.alphaSize; j += 1; {
            if huffman.len[j] == i {
                huffman.table[j] = vec;
                vec += 1;
            }
        });
        vec <<= 1;
    });
}