pub fn bzp_heap_init(huffman: &mut BzpHuffmanInfo) {
    for i in 0..huffman.alpha_size {
        huffman.n_heap += 1;
        huffman.heap[huffman.n_heap] = i as i32;
        bzp_heap_adjust_up(huffman.heap, huffman.weight, huffman.n_heap);
    }
}
