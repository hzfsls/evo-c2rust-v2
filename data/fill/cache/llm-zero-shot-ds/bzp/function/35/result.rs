pub fn bzp_write_select(out_data: &mut BzpOutComdata, huffman: &BzpHuffmanGroups) {
    bzp_write_to_array(huffman.n_select, BZP_BITS15, out_data);
    for i in 0..huffman.n_select {
        for _ in 0..huffman.select_mtf[i] {
            bzp_write_to_array(1, BZP_BIT, out_data);
        }
        bzp_write_to_array(0, BZP_BIT, out_data);
    }
}
