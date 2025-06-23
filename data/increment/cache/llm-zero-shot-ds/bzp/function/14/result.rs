pub fn bzp_generate_select_mtf(huffman: &mut BzpHuffmanGroups) {
    let n_groups = huffman.nGroups;
    let mut list: Vec<i32> = (0..n_groups).collect();
    
    for i in 0..huffman.nSelect {
        let mut pos = 0;
        for (j, &value) in list.iter().enumerate() {
            if huffman.select[i] == value {
                pos = j;
                break;
            }
        }
        
        for j in (1..=pos).rev() {
            list[j] = list[j - 1];
        }
        list[0] = huffman.select[i];
        huffman.selectMTF[i] = pos as i32;
    }
}
