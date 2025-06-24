pub fn BzpBuildHuffmanTree(mut huffman: Ptr<BzpHuffmanInfo>) {
    BzpHuffmanInitArray(huffman);
    BzpHeapInit(huffman);
    let mut idx1: i32;
    let mut idx2: i32;
    while huffman.nHeap > 1 {
        idx1 = huffman.heap[1];
        huffman.heap[1] = huffman.heap[huffman.nHeap];
        huffman.nHeap -= 1;
        BzpHeapAdjustDown(huffman.heap.cast(), huffman.weight.cast(), huffman.nHeap);
        idx2 = huffman.heap[1];
        huffman.heap[1] = huffman.heap[huffman.nHeap];
        huffman.nHeap -= 1;
        BzpHeapAdjustDown(huffman.heap.cast(), huffman.weight.cast(), huffman.nHeap);
        // huffman.weight[huffman.nWeight] = BzpHuffmanWeightAdd(huffman.weight[idx1], huffman.weight[idx2]);
        index!(huffman.weight, huffman.nWeight, BzpHuffmanWeightAdd(huffman.weight[idx1], huffman.weight[idx2]));
        huffman.parent[idx1] = huffman.nWeight;
        huffman.parent[idx2] = huffman.nWeight;
        // huffman.parent[huffman.nWeight] = -1;
        index!(huffman.parent, huffman.nWeight, -1);
        huffman.nHeap += 1;
        // huffman.heap[huffman.nHeap] = huffman.nWeight;
        index!(huffman.heap, huffman.nHeap, huffman.nWeight);
        huffman.nWeight += 1;
        BzpHeapAdjustUp(huffman.heap.cast(), huffman.weight.cast(), huffman.nHeap);
    }
}