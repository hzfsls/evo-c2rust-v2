pub fn BzpDeHuffmanSelect(mut inData: Ptr<InDeComdata>, mut huffman: Ptr<BzpHuffmanDecode>) -> i32 {
    let mut ch: u8 = Default::default();
    let mut selectmtf: Array<i32, { BZP_HUFFMAN_MAX_SIZE_SELECT!() }> = arr![0; BZP_HUFFMAN_MAX_SIZE_SELECT!()];
    c_for!(let mut i: i32 = 0; i < huffman.nSelect.cast(); i.suffix_plus_plus(); {
        let mut j: i32 = -1;
        c_do!({
            ch = BzpReadBits(BZP_BIT!(), inData.cast()).cast();
            j += 1;
        } while ch != 0);
        if (j >= huffman.nGroups.cast()).as_bool() {
            return BZP_ERROR_DATA!();
        }
        selectmtf[i] = j.cast();
    });
    let mut listSelect: Array<i32, { BZP_MAX_GROUPS_NUM!() }> = arr![0; BZP_MAX_GROUPS_NUM!()];
    c_for!(let mut i: i32 = 0; i < BZP_MAX_GROUPS_NUM!().cast(); i.suffix_plus_plus(); {
        listSelect[i] = i.cast();
    });
    c_for!(let mut i: i32 = 0; i < huffman.nSelect.cast(); i.suffix_plus_plus(); {
        let mut pos: i32 = selectmtf[i].cast();
        let mut tmpv: i32 = listSelect[pos].cast();
        c_for!(let mut j: i32 = pos; j > 0; j.prefix_minus_minus(); {
            listSelect[j] = listSelect[j - 1].cast();
        });
        listSelect[0] = tmpv.cast();
        huffman.select[i] = tmpv.cast();
    });
    return BZP_OK!();
}