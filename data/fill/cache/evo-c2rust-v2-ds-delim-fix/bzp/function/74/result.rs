pub fn BzpGetHuffmanTable(mut huffman: Ptr<BzpHuffmanInfo>) {
    let mut vec: i32 = 0;
    let mut mi: i32 = huffman.len[0].cast();
    let mut mx: i32 = huffman.len[0].cast();
    c_for!(let mut i: i32 = 0; i < huffman.alphaSize; i.suffix_plus_plus(); {
        mi = BZP_MIN_FUN!(mi, huffman.len[i]).cast();
        mx = BZP_MAX_FUN!(mx, huffman.len[i]).cast();
    });
    c_for!(let mut i: i32 = mi; i <= mx; i.suffix_plus_plus(); {
        c_for!(let mut j: i32 = 0; j < huffman.alphaSize; j.suffix_plus_plus(); {
            if (huffman.len[j] == i).as_bool() {
                huffman.table[j] = vec.cast();
                vec += 1;
            }
        });
        vec <<= 1;
    });
}
