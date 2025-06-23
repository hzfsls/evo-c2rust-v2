pub fn BzpHeapInit(mut huffman: Ptr<BzpHuffmanInfo>) {
    let mut i: i32 = 0;
    c_for!(i = 0; i < huffman.alphaSize; i += 1; {
        huffman.nHeap += 1;
        index!(huffman.heap, huffman.nHeap, i);
        BzpHeapAdjustUp(huffman.heap.cast(), huffman.weight.cast(), huffman.nHeap);
    });
}