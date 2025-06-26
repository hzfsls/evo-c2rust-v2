pub fn BzpBuildHuffmanTree(mut huffman: Ptr<BzpHuffmanInfo>) {
    BzpHuffmanInitArray(huffman.cast());
    BzpHeapInit(huffman.cast());
    let mut idx1: i32;
    let mut idx2: i32;
    while (huffman.nHeap > 1).as_bool() {
        idx1 = huffman.heap[1].cast();
        huffman.heap[1] = huffman.heap[huffman.nHeap].cast();
        huffman.nHeap -= 1;
        BzpHeapAdjustDown(huffman.heap.cast(), huffman.weight.cast(), huffman.nHeap.cast());
        idx2 = huffman.heap[1].cast();
        huffman.heap[1] = huffman.heap[huffman.nHeap].cast();
        huffman.nHeap -= 1;
        BzpHeapAdjustDown(huffman.heap.cast(), huffman.weight.cast(), huffman.nHeap.cast());
        huffman.weight[huffman.nWeight] = BzpHuffmanWeightAdd(huffman.weight[idx1].cast(), huffman.weight[idx2].cast()).cast();
        huffman.parent[idx1] = huffman.nWeight.cast();
        huffman.parent[idx2] = huffman.nWeight.cast();
        huffman.parent[huffman.nWeight] = -1;
        huffman.nHeap += 1;
        huffman.heap[huffman.nHeap] = huffman.nWeight.cast();
        huffman.nWeight += 1;
        BzpHeapAdjustUp(huffman.heap.cast(), huffman.weight.cast(), huffman.nHeap.cast());
    }
}
