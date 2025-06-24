pub fn BzpDeHuffmanSelect(mut inData: Ptr<InDeComdata>, mut huffman: Ptr<BzpHuffmanDecode>) -> i32 {
    let mut ch: u8;
    let mut selectmtf: Array<i32, { BZP_HUFFMAN_MAX_SIZE_SELECT!() }> = Default::default();
    c_for!(let mut i: i32 = 0; i < huffman.nSelect; i += 1; {
        let mut j: i32 = -1;
        loop {
            ch = BzpReadBits(BZP_BIT!(), inData).cast();
            j += 1;
            if ch == 0 {
                break;
            }
        }
        if j >= huffman.nGroups {
            return BZP_ERROR_DATA!();
        }
        selectmtf[i] = j;
    });
    let mut listSelect: Array<i32, { BZP_MAX_GROUPS_NUM!() }> = Default::default();
    c_for!(let mut i: i32 = 0; i < BZP_MAX_GROUPS_NUM!(); i += 1; {
        listSelect[i] = i;
    });
    c_for!(let mut i: i32 = 0; i < huffman.nSelect; i += 1; {
        let mut pos: i32 = selectmtf[i];
        let mut tmpv: i32 = listSelect[pos];
        c_for!(let mut j: i32 = pos; j > 0; j -= 1; {
            listSelect[j] = listSelect[j - 1];
        });
        listSelect[0] = tmpv;
        huffman.select[i] = tmpv;
    });
    return BZP_OK!();
}