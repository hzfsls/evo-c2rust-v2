pub fn BzpSelectTree(mut huffman: Ptr<BzpHuffmanGroups>) -> i32 {
    let mut id: i32 = 0;
    let mut nGroups: i32 = huffman.nGroups.cast();
    c_for!(let mut k: i32 = 0; k < nGroups; k.suffix_plus_plus(); {
        if huffman.cost[k] < huffman.cost[id] {
            id = k.cast();
        }
    });
    huffman.select[huffman.nSelect.suffix_plus_plus()] = id.cast();
    return id.cast();
}
