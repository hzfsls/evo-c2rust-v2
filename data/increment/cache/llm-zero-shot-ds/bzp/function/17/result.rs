pub fn bzp_select_tree(huffman: &mut BzpHuffmanGroups) -> i32 {
    let mut id = 0;
    let n_groups = huffman.n_groups;
    for k in 0..n_groups {
        if huffman.cost[k] < huffman.cost[id] {
            id = k;
        }
    }
    huffman.select[huffman.n_select] = id;
    huffman.n_select += 1;
    id
}
