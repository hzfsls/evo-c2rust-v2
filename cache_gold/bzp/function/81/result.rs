pub fn BzpCalculateCost(mut huffman: Ptr<BzpHuffmanGroups>, mut st: i32, mut ed: i32) {
    c_memset_s!(huffman.cost, c_sizeofval!(huffman.cost), 0, c_sizeofval!(huffman.cost));
    let mut nGroups: i32 = huffman.nGroups;
    c_for!(let mut k = st; k <= ed; k += 1; {
        c_for!(let mut t = 0; t < nGroups; t += 1; {
            huffman.cost[t] += huffman.huffmanGroups[t].len[huffman.block[k]];
        });
    });
}