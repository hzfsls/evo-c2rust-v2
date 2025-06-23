pub fn bzp_get_huffman_table(huffman: &mut BzpHuffmanInfo) {
    let mut vec = 0;
    let mut mi = huffman.len[0];
    let mut mx = huffman.len[0];
    
    for i in 0..huffman.alpha_size {
        mi = mi.min(huffman.len[i as usize]);
        mx = mx.max(huffman.len[i as usize]);
    }
    
    for i in mi..=mx {
        for j in 0..huffman.alpha_size {
            if huffman.len[j as usize] == i {
                huffman.table[j as usize] = vec;
                vec += 1;
            }
        }
        vec <<= 1;
    }
}
