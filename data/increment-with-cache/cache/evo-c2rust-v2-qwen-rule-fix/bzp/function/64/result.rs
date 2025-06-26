pub fn BzpBuildTreeBalanceHeight(mut huffman: Ptr<BzpHuffmanInfo>) {
    let mut maxlen: i32 = 0;
    c_for!(let mut i: i32 = 0; i < huffman.alphaSize.cast(); i.suffix_plus_plus(); {
        if (huffman.weight[i] == 0).as_bool() {
            huffman.weight[i] = 1 << BZP_HUFFMAN_HEIGHT_WEIGHT_BITS!();
        } else {
            huffman.weight[i] <<= BZP_HUFFMAN_HEIGHT_WEIGHT_BITS!();
        }
    });
    loop {
        maxlen = BzpGetCodeLen(huffman.cast()).cast();
        if (maxlen > BZP_MAX_TREE_HEIGHT_ENCODE!()).as_bool() {
            c_for!(let mut i: i32 = 0; i < huffman.alphaSize.cast(); i.suffix_plus_plus(); {
                let mut w: i32 = (huffman.weight[i] >> BZP_HUFFMAN_HEIGHT_WEIGHT_BITS!()).cast();
                w = ((w >> 1) + 1).cast();
                huffman.weight[i] = w << BZP_HUFFMAN_HEIGHT_WEIGHT_BITS!();
            });
        } else {
            break;
        }
    }
}