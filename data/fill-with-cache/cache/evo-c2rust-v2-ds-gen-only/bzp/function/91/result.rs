pub fn BzpHuffmanDecodeFinish(mut huffman: Ptr<BzpHuffmanDecode>) {
    if (huffman != NULL!()).as_bool() {
        if (huffman.select != NULL!()).as_bool() {
            c_free!(huffman.select);
            huffman.select = NULL!();
        }
        c_free!(huffman);
        huffman = NULL!();
    }
}
