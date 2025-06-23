pub fn bzp_write_len(out_data: &mut BzpOutComdata, huffman: &BzpHuffmanGroups) {
    for i in 0..huffman.n_groups {
        let mut val = huffman.huffman_groups[i].len[0];
        bzp_write_to_array(val, BZP_BITS5, out_data);

        for j in 0..huffman.alpha_size {
            let tar = huffman.huffman_groups[i].len[j];
            let (mut deta, mut save_val) = (0, 0);
            if val < tar {
                save_val = BZP_HUFFMAN_LEN_INCREASE;
                deta = 1;
            } else if val > tar {
                save_val = BZP_HUFFMAN_LEN_REDUCED;
                deta = -1;
            }
            while val != tar {
                bzp_write_to_array(save_val, BZP_BITS2, out_data);
                val += deta;
            }
            bzp_write_to_array(0, BZP_BIT, out_data);
        }
    }
}
