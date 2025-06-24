pub fn BzpHuffmanInitArray(mut huffman: Ptr<BzpHuffmanInfo>) {
    let mut i: i32;
    huffman.nHeap = 0;
    huffman.nWeight = huffman.alphaSize;
    c_for!(i = 0; i < huffman.alphaSize; i += 1; {
        huffman.parent[i] = -1;
    });
}