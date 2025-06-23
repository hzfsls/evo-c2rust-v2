pub fn bzp_get_one_table(huffman: &mut BzpHuffmanDecode, t: i32) {
    let mut vec = 0;
    let mut cnt = 0;
    let mut mi = huffman.len[t as usize][0];
    let mut mx = huffman.len[t as usize][0];
    
    for i in 0..huffman.alpha_size {
        mi = mi.min(huffman.len[t as usize][i as usize]);
        mx = mx.max(huffman.len[t as usize][i as usize]);
    }
    
    huffman.min_lens[t as usize] = mi;
    
    for i in mi..=mx {
        for j in 0..huffman.alpha_size {
            if huffman.len[t as usize][j as usize] == i {
                huffman.perm[t as usize][cnt as usize] = j;
                cnt += 1;
            }
        }
    }
    
    for i in 0..huffman.alpha_size {
        let len = huffman.len[t as usize][i as usize] + 1;
        huffman.base[t as usize][len as usize] += 1;
    }
    
    for i in 1..=mx + 1 {
        huffman.base[t as usize][i as usize] += huffman.base[t as usize][(i - 1) as usize];
    }
    
    for i in mi..=mx {
        vec += huffman.base[t as usize][(i + 1) as usize] - huffman.base[t as usize][i as usize];
        huffman.limit[t as usize][i as usize] = vec - 1;
        vec <<= 1;
    }
    
    for i in mi + 1..=mx {
        huffman.base[t as usize][i as usize] = 
            ((huffman.limit[t as usize][(i - 1) as usize] + 1) << 1) - huffman.base[t as usize][i as usize];
    }
}
