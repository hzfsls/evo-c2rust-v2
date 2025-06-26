pub struct BzpHuffmanGroups {
    pub block: *mut i32,
    pub mtf_freq: *mut i32,
    pub select: *mut i32,
    pub select_mtf: *mut i32,
    pub huffman_groups: [BzpHuffmanInfo; BZP_MAX_GROUPS_NUM],
    pub cost: [i32; BZP_MAX_GROUPS_NUM],
    pub n_groups: i32,
    pub n_block: i32,
    pub n_select: i32,
    pub alpha_size: i32,
}
