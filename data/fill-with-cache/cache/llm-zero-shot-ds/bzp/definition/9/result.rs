pub struct BzpHuffmanInfo {
    heap: [i32; BZP_MAX_ALPHA_SIZE + 1],
    weight: [i32; BZP_MAX_ALPHA_SIZE * 2],
    parent: [i32; BZP_MAX_ALPHA_SIZE * 2],
    len: [i32; BZP_MAX_ALPHA_SIZE],
    table: [i32; BZP_MAX_ALPHA_SIZE],
    n_heap: i32,
    n_weight: i32,
    alpha_size: i32,
}
