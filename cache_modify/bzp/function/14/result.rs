pub fn BzpGenerateSelectMTF(mut huffman: Ptr<BzpHuffmanGroups>) {
    let mut nGroups: i32 = huffman.nGroups;
    let mut list: Vec<i32> = vec![0; nGroups as usize];
    c_for!(let mut i: i32 = 0; i < nGroups; i.suffix_plus_plus(); {
        list[i as usize] = i;
    });
    c_for!(let mut i: i32 = 0; i < huffman.nSelect; i.suffix_plus_plus(); {
        let mut pos: i32 = 0;
        c_for!(let mut j: i32 = 0; j < nGroups; j.suffix_plus_plus(); {
            if (huffman.select[i] == list[j as usize]) {
                pos = j;
                break;
            }
        });
        c_for!(let mut j: i32 = pos; j > 0; j.suffix_minus_minus(); {
            list[j as usize] = list[(j - 1) as usize];
        });
        let tmp0 = 0;
        list[tmp0 as usize];
        huffman.selectMTF[i] = pos;
    });
}
