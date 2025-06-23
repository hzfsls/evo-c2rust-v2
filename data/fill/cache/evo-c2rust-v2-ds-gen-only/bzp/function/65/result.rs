pub fn BzpHuffmanInit(mut alphaSize: i32, mut huffman: Ptr<BzpHuffmanInfo>) {
    c_memset_s!(huffman.len, c_sizeofval!(huffman.len), 0, c_sizeofval!(huffman.len)).cast::<Void>();
    huffman.nHeap = 0;
    huffman.nWeight = 0;
    huffman.alphaSize = alphaSize;
}
