pub fn BzpBuildHuffmanTree(mut huffman: Ptr<BzpHuffmanInfo>) {
    BzpHuffmanInitArray(huffman.cast());
    BzpHeapInit(huffman.cast());
    let mut idx1: i32 = Default::default();
    let mut idx2: i32 = Default::default();
    while (huffman.nHeap > 1).as_bool() {
        idx1 = huffman.heap[1];
        huffman.heap[1] = huffman.heap[huffman.nHeap];
        huffman.nHeap -= 1;
        BzpHeapAdjustDown(huffman.heap.cast(), huffman.weight.cast(), huffman.nHeap.cast());
        idx2 = huffman.heap[1];
        huffman.heap[1] = huffman.heap[huffman.nHeap];
        huffman.nHeap -= 1;
        BzpHeapAdjustDown(huffman.heap.cast(), huffman.weight.cast(), huffman.nHeap.cast());
        huffman.weight[huffman.nWeight] = BzpHuffmanWeightAdd(huffman.weight[idx1].cast(), huffman.weight[idx2].cast()).cast();
        huffman.parent[idx1] = huffman.nWeight;
        huffman.parent[idx2] = huffman.nWeight;
        huffman.parent[huffman.nWeight] = -1;
        huffman.nHeap += 1;
        huffman.heap[huffman.nHeap] = huffman.nWeight;
        huffman.nWeight += 1;
        BzpHeapAdjustUp(huffman.heap.cast(), huffman.weight.cast(), huffman.nHeap.cast());
    }
}