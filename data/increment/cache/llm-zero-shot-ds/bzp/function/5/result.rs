pub fn bzp_huffman_weight_add(w1: i32, w2: i32) -> i32 {
    ((w1 & 0xffffff00) + (w2 & 0xffffff00)) | (std::cmp::max(w1 & 0x000000ff, w2 & 0x000000ff) + 1)
}
