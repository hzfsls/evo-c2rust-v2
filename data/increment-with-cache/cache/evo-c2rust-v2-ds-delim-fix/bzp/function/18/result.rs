pub fn BzpHuffmanMain(mut huffman: Ptr<BzpHuffmanGroups>) {
    let mut nGroups: i32 = BzpGetHuffmanGroups(huffman.nBlock.cast()).cast();
    huffman.nGroups = nGroups.cast();

    BzpInitLenArray(huffman.cast());
    let mut st: i32 = 0;
    let mut ed: i32 = Default::default();

    c_for!(let mut i: i32 = 0; i < BZP_MAX_ITER_NUM!().cast(); i.suffix_plus_plus(); {
        c_for!(let mut j: i32 = 0; j < nGroups; j.suffix_plus_plus(); {
            c_memset_s!(huffman.huffmanGroups[j].weight, c_sizeofval!(huffman.huffmanGroups[j].weight), 0, c_sizeofval!(huffman.huffmanGroups[j].weight)).cast::<Void>();
        });

        st = 0;
        huffman.nSelect = 0;
        while (st < huffman.nBlock).as_bool() {
            ed = BZP_MIN_FUN!(huffman.nBlock, st + BZP_ELEMS_NUM_IN_ONE_GROUP!().cast()).cast::<i32>() - 1;

            BzpCalculateCost(huffman.cast(), st.cast(), ed.cast());

            let mut id: i32 = BzpSelectTree(huffman.cast()).cast();

            c_for!(let mut k: i32 = st; k <= ed; k.suffix_plus_plus(); {
                huffman.huffmanGroups[id].weight[huffman.block[k]] += 1;
            });
            st = ed + 1;
        }

        c_for!(let mut j: i32 = 0; j < nGroups; j.suffix_plus_plus(); {
            BzpBuildTreeBalanceHeight(c_ref!(huffman.huffmanGroups[j]).cast());
        });
    });

    BzpGenerateSelectMTF(huffman.cast());

    c_for!(let mut i: i32 = 0; i < nGroups; i.suffix_plus_plus(); {
        BzpGetHuffmanTable(c_ref!(huffman.huffmanGroups[i]).cast());
    });
}
