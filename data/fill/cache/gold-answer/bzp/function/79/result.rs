pub fn BzpGenerateSelectMTF(mut huffman: Ptr<BzpHuffmanGroups>) {
    let mut nGroups: i32 = huffman.nGroups;
    let mut list = vec![0; nGroups as usize];
    c_for!(let mut i = 0; i < nGroups; i += 1; {
        list[i as usize] = i;
    });
    c_for!(let mut i = 0; i < huffman.nSelect; i += 1; {
        let mut pos: i32 = 0;
        c_for!(let mut j = 0; j < nGroups; j += 1; {
            if huffman.select[i] == list[j as usize] {
                pos = j;
                break;
            }
        });
        c_for!(let mut j = pos; j > 0; j -= 1; {
            list[j as usize] = list[(j - 1) as usize];
        });
        list[0] = huffman.select[i];
        huffman.selectMTF[i] = pos;
    });
}