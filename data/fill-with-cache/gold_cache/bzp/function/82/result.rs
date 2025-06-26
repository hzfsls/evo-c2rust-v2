pub fn BzpSelectTree(mut huffman: Ptr<BzpHuffmanGroups>) -> i32 {
    let mut id: i32 = 0;
    let mut nGroups: i32 = huffman.nGroups;
    c_for!(let mut k = 0; k < nGroups; k += 1; {
        if huffman.cost[k] < huffman.cost[id] {
            id = k;
        }
    });
    // huffman.select[huffman.nSelect] = id;
    index!(huffman.select, huffman.nSelect, id);
    huffman.nSelect += 1;
    return id;
}