pub fn bzp_write_input_encode(
    out_data: &mut BzpOutComdata,
    mtf: &BzpMtfInfo,
    huffman: &BzpHuffmanGroups,
) {
    for i in 0..mtf.n_mtf {
        let val = mtf.mtf_v[i];
        let gid = huffman.select[i / BZP_ELEMS_NUM_IN_ONE_GROUP];
        let code = huffman.huffman_groups[gid as usize].table[val as usize];
        let len = huffman.huffman_groups[gid as usize].len[val as usize];
        bzp_write_to_array(code, len, out_data);
    }
}
