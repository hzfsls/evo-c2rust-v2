pub fn bzp_build_tree_balance_height(huffman: &mut BzpHuffmanInfo) {
    let mut maxlen;
    for i in 0..huffman.alpha_size {
        if huffman.weight[i] == 0 {
            huffman.weight[i] = 1 << BZP_HUFFMAN_HEIGHT_WEIGHT_BITS;
        } else {
            huffman.weight[i] <<= BZP_HUFFMAN_HEIGHT_WEIGHT_BITS;
        }
    }

    loop {
        maxlen = bzp_get_code_len(huffman);

        if maxlen > BZP_MAX_TREE_HEIGHT_ENCODE {
            for i in 0..huffman.alpha_size {
                let mut w = huffman.weight[i] >> BZP_HUFFMAN_HEIGHT_WEIGHT_BITS;
                w = (w >> 1) + 1;
                huffman.weight[i] = w << BZP_HUFFMAN_HEIGHT_WEIGHT_BITS;
            }
        } else {
            break;
        }
    }
}
