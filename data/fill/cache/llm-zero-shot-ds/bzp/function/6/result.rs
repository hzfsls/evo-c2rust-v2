pub fn bzp_huffman_decode_step(huffman: &mut BzpHuffmanDecode, in_data: &mut InDeComdata) -> i32 {
    if huffman.de_code_num == BZP_ELEMS_NUM_IN_ONE_GROUP {
        huffman.de_code_num = 0;
        huffman.select_cnt += 1;
    }
    let gid = huffman.select[huffman.select_cnt as usize];
    let mut chlen = huffman.min_lens[gid as usize];
    let mut val = bzp_read_bits(chlen, in_data);
    while chlen < BZP_HUFFMAN_LEN_UPPER_LIMIT && val > huffman.limit[gid as usize][chlen as usize] {
        chlen += 1;
        let nxtbit = bzp_read_bits(1, in_data);
        val = (val << 1) | nxtbit;
    }
    if chlen > BZP_HUFFMAN_LEN_UPPER_LIMIT {
        return -1;
    }
    val = val - huffman.base[gid as usize][chlen as usize];
    val = huffman.perm[gid as usize][val as usize];
    huffman.de_code_num += 1;
    val
}
