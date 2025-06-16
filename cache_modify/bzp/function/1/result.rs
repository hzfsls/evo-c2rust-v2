pub fn BzpHuffmanInitArray(mut huffman: Ptr<BzpHuffmanInfo>) {
    let mut i: i32;
    huffman.nHeap = 0;
    huffman.nWeight = huffman.alphaSize.cast();

    c_for!(i = 0; i < huffman.alphaSize; i.suffix_plus_plus(); {
        huffman.parent[i] = -1;
    });
}
