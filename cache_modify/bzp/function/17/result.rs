pub fn BzpSelectTree(mut huffman: Ptr<BzpHuffmanGroups>) -> i32 {
    let mut id: i32 = 0;
    let mut nGroups: i32 = huffman.nGroups;
    c_for!(let mut k: i32 = 0; k < nGroups; k.suffix_plus_plus(); {
        let tmp0 = k;
        if (huffman.cost[tmp0]).as_bool() {
            id = k;
        }
    });
    let mut tmp0: i32 = huffman.nSelect;
    huffman.select[tmp0] = id;
    huffman.nSelect += 1;
    return id;
}
