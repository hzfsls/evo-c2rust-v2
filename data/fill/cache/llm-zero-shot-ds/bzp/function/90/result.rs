pub fn bzp_generate_decode_table(huffman: &mut BzpHuffmanDecode) {
    for t in 0..huffman.n_groups {
        bzp_get_one_table(huffman, t);
    }
}
