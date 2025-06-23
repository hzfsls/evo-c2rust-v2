pub fn bzp_generate_select_mtf(huffman: &mut BzpHuffmanGroups) {
    let n_groups = huffman.nGroups;
    let mut list: Vec<i32> = (0..n_groups).collect();
    
    for i in 0..huffman.nSelect {
        let select_val = huffman.select[i];
        let pos = list.iter().position(|&x| x == select_val).unwrap_or(0);
        
        for j in (1..=pos).rev() {
            list[j] = list[j - 1];
        }
        list[0] = select_val;
        huffman.selectMTF[i] = pos as i32;
    }
}
