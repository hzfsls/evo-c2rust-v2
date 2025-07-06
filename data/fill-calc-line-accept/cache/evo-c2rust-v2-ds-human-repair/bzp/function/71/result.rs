pub fn BzpBuildHuffmanTree(mut huffman: Ptr<BzpHuffmanInfo>) {
    BzpHuffmanInitArray(huffman);
    BzpHeapInit(huffman);
    let mut idx1: i32;
    let mut idx2: i32;
    while (huffman.nHeap > 1) {
        idx1 = huffman.heap[1];
        let tmp0 = 1;
        huffman.heap[tmp0] =  huffman.heap[huffman.nHeap];
        huffman.nHeap -= 1;
        BzpHeapAdjustDown(huffman.heap.cast(), huffman.weight.cast(), huffman.nHeap);
        idx2 = huffman.heap[1];
        let tmp0 = huffman.nHeap;
        huffman.heap[1] = huffman.heap[tmp0];
        huffman.nHeap -= 1;
        BzpHeapAdjustDown(huffman.heap.cast(), huffman.weight.cast(), huffman.nHeap);
        let tmp0 = huffman.nWeight;
        huffman.weight[tmp0] = BzpHuffmanWeightAdd(huffman.weight[idx1], huffman.weight[idx2]);
        huffman.parent[idx1] = huffman.nWeight;
        huffman.parent[idx2] = huffman.nWeight;
        let tmp0 = huffman.nWeight;
        huffman.parent[tmp0] = -1;
        huffman.nHeap += 1;
        let tmp0 = huffman.nHeap;
        huffman.heap[tmp0] = huffman.nWeight;
        huffman.nWeight += 1;
        BzpHeapAdjustUp(huffman.heap.cast(), huffman.weight.cast(), huffman.nHeap);
    }
}
