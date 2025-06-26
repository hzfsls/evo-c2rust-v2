pub fn BzpHuffmanDecodeFinish(mut huffman: Ptr<BzpHuffmanDecode>) {
    if (huffman != NULL!()).as_bool() {
        if (huffman.select != NULL!()).as_bool() {
            c_free!(huffman.select.cast());
            huffman.select = NULL!();
        }
        c_free!(huffman.cast());
        huffman = NULL!();
    }
}