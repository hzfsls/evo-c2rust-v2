pub fn BzpHeapInit(mut huffman: Ptr<BzpHuffmanInfo>) {
    let mut i: i32 = 0;
    c_for!(i = 0; i < huffman.alphaSize; i.suffix_plus_plus(); {
        huffman.nHeap += 1;
        let tmp0 = huffman.nHeap;
        huffman.heap[tmp0] = i;
        BzpHeapAdjustUp(huffman.heap.cast(), huffman.weight.cast(), huffman.nHeap);
    });
}
