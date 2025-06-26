pub fn BzpBuildTreeBalanceHeight(mut huffman: Ptr<BzpHuffmanInfo>) {
    let mut maxlen: i32 = 0;
    c_for!(let mut i: i32 = 0; i < huffman.alphaSize; i += 1; {
        if huffman.weight[i] == 0 {
            huffman.weight[i] = 1 << BZP_HUFFMAN_HEIGHT_WEIGHT_BITS!();
        } else {
            huffman.weight[i] <<= BZP_HUFFMAN_HEIGHT_WEIGHT_BITS!();
        }
    });
    loop {
        maxlen = BzpGetCodeLen(huffman);
        if maxlen <= BZP_MAX_TREE_HEIGHT_ENCODE!() {
            break;
        }
        c_for!(let mut i: i32 = 0; i < huffman.alphaSize; i += 1; {
            let mut w: i32 = huffman.weight[i] >> BZP_HUFFMAN_HEIGHT_WEIGHT_BITS!();
            w = (w >> 1) + 1;
            huffman.weight[i] = w << BZP_HUFFMAN_HEIGHT_WEIGHT_BITS!();
        });
    }
}