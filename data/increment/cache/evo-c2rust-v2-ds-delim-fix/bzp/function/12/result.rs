pub fn BzpBzpHuffmanGroupsFinish(mut huffman: Ptr<BzpHuffmanGroups>) {
    if (huffman != NULL!()).as_bool() {
        if (huffman.select != NULL!()).as_bool() {
            c_free!(huffman.select);
            huffman.select = NULL!();
        }
        if (huffman.selectMTF != NULL!()).as_bool() {
            c_free!(huffman.selectMTF);
            huffman.selectMTF = NULL!();
        }
        c_free!(huffman);
        huffman = NULL!();
    }
}
