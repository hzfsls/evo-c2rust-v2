pub fn bz_huffman_init_array(huffman: &mut BzpHuffmanInfo) {
    huffman.n_heap = 0;
    huffman.n_weight = huffman.alpha_size;

    for i in 0..huffman.alpha_size {
        huffman.parent[i] = -1;
    }
}
