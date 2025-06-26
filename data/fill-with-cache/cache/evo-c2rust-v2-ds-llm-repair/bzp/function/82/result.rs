pub fn BzpSelectTree(mut huffman: Ptr<BzpHuffmanGroups>) -> i32 {
    let mut id: i32 = 0;
    let mut nGroups: i32 = huffman.nGroups;
    c_for!(let mut k: i32 = 0; k < nGroups; k.suffix_plus_plus(); {
        let tmp0 = k;
        if (huffman.cost[tmp0]) {
            id = k;
        }
    });
    huffman.select[huffman.nSelect] = id;
    huffman.nSelect += 1;
    return id;
}
