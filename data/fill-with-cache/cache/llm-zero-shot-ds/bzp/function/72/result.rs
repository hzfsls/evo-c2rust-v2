use std::cmp::max;

pub struct BzpHuffmanInfo {
    alpha_size: i32,
    parent: Vec<i32>,
    len: Vec<i32>,
    // Assuming other necessary fields are present
}

pub fn bzp_get_code_len(huffman: &mut BzpHuffmanInfo) -> i32 {
    // Assuming BzpBuildHuffmanTree is implemented elsewhere
    // BzpBuildHuffmanTree(huffman);
    
    let mut max_len = 0;
    
    for i in 0..huffman.alpha_size {
        let mut x = i;
        let mut t_len = 0;
        
        while huffman.parent[x as usize] >= 0 {
            x = huffman.parent[x as usize];
            t_len += 1;
        }
        
        huffman.len[i as usize] = t_len;
        max_len = max(max_len, t_len);
    }
    
    max_len
}
