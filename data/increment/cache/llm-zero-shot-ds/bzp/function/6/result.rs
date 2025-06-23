pub fn bzp_build_huffman_tree(huffman: &mut BzpHuffmanInfo) {
    bzp_huffman_init_array(huffman);
    bzp_heap_init(huffman);
    
    let mut idx1;
    let mut idx2;
    
    while huffman.n_heap > 1 {
        idx1 = huffman.heap[1];
        huffman.heap[1] = huffman.heap[huffman.n_heap];
        huffman.n_heap -= 1;
        bzp_heap_adjust_down(&mut huffman.heap, &huffman.weight, huffman.n_heap);
        
        idx2 = huffman.heap[1];
        huffman.heap[1] = huffman.heap[huffman.n_heap];
        huffman.n_heap -= 1;
        bzp_heap_adjust_down(&mut huffman.heap, &huffman.weight, huffman.n_heap);
        
        huffman.weight[huffman.n_weight] = bzp_huffman_weight_add(huffman.weight[idx1], huffman.weight[idx2]);
        huffman.parent[idx1] = huffman.n_weight;
        huffman.parent[idx2] = huffman.n_weight;
        huffman.parent[huffman.n_weight] = -1;
        
        huffman.n_heap += 1;
        huffman.heap[huffman.n_heap] = huffman.n_weight;
        huffman.n_weight += 1;
        
        bzp_heap_adjust_up(&mut huffman.heap, &huffman.weight, huffman.n_heap);
    }
}
