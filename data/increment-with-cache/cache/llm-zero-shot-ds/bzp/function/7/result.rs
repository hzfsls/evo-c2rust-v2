use std::cmp::max;

pub struct BzpHuffmanInfo {
    alphaSize: i32,
    parent: Vec<i32>,
    len: Vec<i32>,
    // Assuming other necessary fields are present
}

pub fn bzp_get_code_len(huffman: &mut BzpHuffmanInfo) -> i32 {
    // Assuming BzpBuildHuffmanTree is defined elsewhere
    // BzpBuildHuffmanTree(huffman);
    
    let mut maxlen = 0;
    
    for i in 0..huffman.alphaSize {
        let mut x = i;
        let mut tlen = 0;
        while huffman.parent[x as usize] >= 0 {
            x = huffman.parent[x as usize];
            tlen += 1;
        }
        huffman.len[i as usize] = tlen;
        maxlen = max(maxlen, tlen);
    }
    
    maxlen
}
