pub fn BzpHuffmanDecodeFinish(mut huffman: Ptr<BzpHuffmanDecode>) {
    if huffman != NULL!() {
        if huffman.select != NULL!() {
            c_free!(huffman.select);
            huffman.select = NULL!();
        }
        c_free!(huffman);
        huffman = NULL!();
    }
}
