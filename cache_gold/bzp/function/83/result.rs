pub fn BzpHuffmanMain(mut huffman: Ptr<BzpHuffmanGroups>) {
    let mut nGroups: i32 = BzpGetHuffmanGroups(huffman.nBlock);
    huffman.nGroups = nGroups;
    BzpInitLenArray(huffman);
    let mut st: i32 = 0;
    let mut ed: i32;
    c_for!(let mut i = 0; i < BZP_MAX_ITER_NUM!(); i += 1; {
        c_for!(let mut j = 0; j < nGroups; j += 1; {
            c_memset_s!(huffman.huffmanGroups[j].weight, c_sizeofval!(huffman.huffmanGroups[j].weight), 0, c_sizeofval!(huffman.huffmanGroups[j].weight));
        });
        st = 0;
        huffman.nSelect = 0;
        while st < huffman.nBlock {
            ed = BZP_MIN_FUN!(huffman.nBlock, st + BZP_ELEMS_NUM_IN_ONE_GROUP!()) - 1;
            BzpCalculateCost(huffman, st, ed);
            let mut id: i32 = BzpSelectTree(huffman);
            c_for!(let mut k = st; k <= ed; k += 1; {
                // huffman.huffmanGroups[id].weight[huffman.block[k]] += 1;
                let index = huffman.block[k];
                huffman.huffmanGroups[id].weight[index] += 1;
            });
            st = ed + 1;
        }
        c_for!(let mut j = 0; j < nGroups; j += 1; {
            BzpBuildTreeBalanceHeight(c_ref!(huffman.huffmanGroups[j]));
        });
    });
    BzpGenerateSelectMTF(huffman);
    c_for!(let mut i = 0; i < nGroups; i += 1; {
        BzpGetHuffmanTable(c_ref!(huffman.huffmanGroups[i]));
    });
}