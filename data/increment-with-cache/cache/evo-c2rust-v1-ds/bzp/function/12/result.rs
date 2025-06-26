pub fn BzpBzpHuffmanGroupsFinish(mut huffman: Ptr<BzpHuffmanGroups>) {
    if huffman != NULL!() {
        if huffman.select != NULL!() {
            c_free!(huffman.select);
            huffman.select = NULL!();
        }
        if huffman.selectMTF != NULL!() {
            c_free!(huffman.selectMTF);
            huffman.selectMTF = NULL!();
        }
        c_free!(huffman);
        huffman = NULL!();
    }
}
