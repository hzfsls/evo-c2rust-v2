pub fn BzpGetCodeLen(mut huffman: Ptr<BzpHuffmanInfo>) -> i32 {
    let mut maxlen: i32 = 0;
    BzpBuildHuffmanTree(huffman.cast());
    let mut i: i32;
    maxlen = 0;
    c_for!(i = 0; i < huffman.alphaSize; i.suffix_plus_plus(); {
        let mut x: i32 = i.cast();
        let mut tlen: i32 = 0;
        while (huffman.parent[x] >= 0).as_bool() {
            x = huffman.parent[x].cast();
            tlen += 1;
        }
        huffman.len[i] = tlen.cast();
        maxlen = BZP_MAX_FUN!(maxlen, tlen);
    });
    return maxlen.cast();
}
