use std::cmp::{min, max};

pub struct BzpHuffmanInfo {
    pub len: Vec<i32>,
    pub alpha_size: i32,
    pub table: Vec<i32>,
}

pub fn bzp_get_huffman_table(huffman: &mut BzpHuffmanInfo) {
    let mut vec = 0;
    let mut mi = huffman.len[0];
    let mut mx = huffman.len[0];
    
    for i in 0..huffman.alpha_size as usize {
        mi = min(mi, huffman.len[i]);
        mx = max(mx, huffman.len[i]);
    }
    
    for i in mi..=mx {
        for j in 0..huffman.alpha_size as usize {
            if huffman.len[j] == i {
                huffman.table[j] = vec;
                vec += 1;
            }
        }
        vec <<= 1;
    }
}
