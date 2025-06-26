pub fn bzp_init_len_array(huffman: &mut BzpHuffmanGroups) {
    let n_groups = huffman.nGroups;
    let mut n_part = n_groups;
    let mut all_freq_num = huffman.nBlock;
    let mut st = 0;
    let mut ed;

    while n_part > 0 {
        let mut now_freq_num = 0;
        let freq_num_limit = all_freq_num / n_part;

        ed = st - 1;
        while ed < huffman.alphaSize - 1 && now_freq_num < freq_num_limit {
            ed += 1;
            now_freq_num += huffman.mtfFreq[ed as usize];
        }

        if ed > st && n_part != n_groups && n_part != 1 && ((n_groups - n_part) & 1) != 0 {
            now_freq_num -= huffman.mtfFreq[ed as usize];
            ed -= 1;
        }

        for i in 0..huffman.alphaSize {
            if i >= st && i <= ed {
                huffman.huffmanGroups[(n_part - 1) as usize].len[i as usize] = 0;
            } else {
                huffman.huffmanGroups[(n_part - 1) as usize].len[i as usize] = BZP_HUFFMAN_LEN_MAX_COST;
            }
        }
        n_part -= 1;
        st = ed + 1;
        all_freq_num -= now_freq_num;
    }
}
