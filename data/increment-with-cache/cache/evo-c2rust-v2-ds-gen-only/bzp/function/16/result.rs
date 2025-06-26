pub fn BzpCalculateCost(mut huffman: Ptr<BzpHuffmanGroups>, mut st: i32, mut ed: i32) {
    c_memset_s!(huffman.cost, c_sizeofval!(huffman.cost), 0, c_sizeofval!(huffman.cost)).cast::<Void>();
    let mut nGroups: i32 = huffman.nGroups.cast();
    c_for!(let mut k: i32 = st; k <= ed; k.suffix_plus_plus(); {
        c_for!(let mut t: i32 = 0; t < nGroups; t.suffix_plus_plus(); {
            huffman.cost[t] += huffman.huffmanGroups[t].len[huffman.block[k]].cast();
        });
    });
}
