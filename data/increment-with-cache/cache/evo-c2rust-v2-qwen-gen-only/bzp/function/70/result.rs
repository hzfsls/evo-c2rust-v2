pub fn BzpGenerateSelectMTF(mut huffman: Ptr<BzpHuffmanGroups>) {
    let mut nGroups: i32 = huffman.nGroups.cast();
    let mut list: Array<i32, { BZP_GROUPS_ASCII!() }> = Default::default();
    c_for!(let mut i: i32 = 0; i < nGroups.cast(); i.suffix_plus_plus(); {
        list[i] = i.cast();
    });
    c_for!(let mut i: i32 = 0; i < huffman.nSelect.cast(); i.suffix_plus_plus(); {
        let mut pos: i32 = 0;
        c_for!(let mut j: i32 = 0; j < nGroups.cast(); j.suffix_plus_plus(); {
            if (huffman.select[i] == list[j]).as_bool() {
                pos = j.cast();
                break;
            }
        });
        c_for!(let mut j: i32 = pos; j > 0; j.prefix_minus_minus(); {
            list[j] = list[j - 1].cast();
        });
        list[0] = huffman.select[i].cast();
        huffman.selectMTF[i] = pos.cast();
    });
}