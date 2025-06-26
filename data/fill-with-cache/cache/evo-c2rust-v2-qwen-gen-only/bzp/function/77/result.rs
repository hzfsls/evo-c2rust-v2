pub fn BzpHuffmanGroupsFinish(mut huffman: Ptr<BzpHuffmanGroups>) {
    if (huffman != NULL!()).as_bool() {
        if (huffman.select != NULL!()).as_bool() {
            c_free!(huffman.select.cast());
            huffman.select = NULL!();
        }
        if (huffman.selectMTF != NULL!()).as_bool() {
            c_free!(huffman.selectMTF.cast());
            huffman.selectMTF = NULL!();
        }
        c_free!(huffman.cast());
        huffman = NULL!();
    }
}