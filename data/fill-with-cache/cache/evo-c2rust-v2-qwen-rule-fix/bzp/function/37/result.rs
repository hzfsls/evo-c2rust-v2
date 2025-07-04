pub fn BzpWriteInputEncode(mut outData: Ptr<BzpOutComdata>, mut mtf: Ptr<BzpMtfInfo>, mut huffman: Ptr<BzpHuffmanGroups>) {
    c_for!(let mut i: i32 = 0; i < mtf.nMtf; i.suffix_plus_plus(); {
        let mut val: i32 = mtf.mtfV[i];
        let mut gid: i32 = huffman.select[i / BZP_ELEMS_NUM_IN_ONE_GROUP!()];
        let mut code: i32 = huffman.huffmanGroups[gid].table[val];
        let mut len: i32 = huffman.huffmanGroups[gid].len[val];
        BzpWriteToArray(code, len, outData);
    });
}