pub fn BzpGenerateSelectMTF(mut huffman: Ptr<BzpHuffmanGroups>) {
    let mut nGroups: i32 = huffman.nGroups;
    let mut list: Array<i32, { nGroups }> = arr![0; nGroups];
    c_for!(let mut i: i32 = 0; i < nGroups; i.suffix_plus_plus(); {
        list[i] = i;
    });
    c_for!(let mut i: i32 = 0; i < huffman.nSelect; i.suffix_plus_plus(); {
        let mut pos: i32 = 0;
        c_for!(let mut j: i32 = 0; j < nGroups; j.suffix_plus_plus(); {
            if (huffman.select[i] == list[j]) {
                pos = j;
                break;
            }
        });
        c_for!(let mut j: i32 = pos; j > 0; j.suffix_minus_minus(); {
            list[j] = list[j - 1];
        });
        let tmp0 = 0;
        list[tmp0];
        huffman.selectMTF[i] = pos;
    });
}
