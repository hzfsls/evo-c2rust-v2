pub fn BzpHuffmanGroupsInit(mut blockSize: i32) -> Ptr<BzpHuffmanGroups> {
    if BZP_INVALID_BLOCK_SIZE!(blockSize).as_bool() {
        return NULL!();
    }
    let mut huffmanGroups: Ptr<BzpHuffmanGroups> = c_malloc!(c_sizeof!(BzpHuffmanGroups));
    if (huffmanGroups == NULL!()).as_bool() {
        return NULL!();
    }
    huffmanGroups.select = NULL!();
    huffmanGroups.selectMTF = NULL!();
    let mut spaceSize: i32 = blockSize * BZP_BASE_BLOCK_SIZE!() / BZP_ELEMS_NUM_IN_ONE_GROUP!();
    huffmanGroups.select = c_malloc!(spaceSize * c_sizeof!(i32));
    huffmanGroups.selectMTF = c_malloc!(spaceSize * c_sizeof!(i32));
    if (huffmanGroups.select == NULL!()).as_bool() || (huffmanGroups.selectMTF == NULL!()).as_bool() {
        BzpBzpHuffmanGroupsFinish(huffmanGroups.cast());
        return NULL!();
    }
    huffmanGroups.alphaSize = 0;
    huffmanGroups.block = NULL!();
    huffmanGroups.mtfFreq = NULL!();
    huffmanGroups.nSelect = 0;
    huffmanGroups.nGroups = 0;

    c_for!(let mut i: i32 = 0; i < BZP_MAX_GROUPS_NUM!(); i.suffix_plus_plus(); {
        BzpHuffmanInit(0, c_ref!(huffmanGroups.huffmanGroups[i]).cast());
    });

    return huffmanGroups.cast();
}
