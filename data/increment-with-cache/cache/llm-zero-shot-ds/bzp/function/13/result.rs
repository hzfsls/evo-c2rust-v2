pub fn bzp_get_huffman_groups(n_block: i32) -> i32 {
    let mut n_groups = 1;
    if n_block < BZP_NGROUPS_BLOCK_NUM_LIMIT0 {
        n_groups = BZP_NGROUPS_NUM_0;
    } else if n_block < BZP_NGROUPS_BLOCK_NUM_LIMIT1 {
        n_groups = BZP_NGROUPS_NUM_1;
    } else if n_block < BZP_NGROUPS_BLOCK_NUM_LIMIT2 {
        n_groups = BZP_NGROUPS_NUM_2;
    } else if n_block < BZP_NGROUPS_BLOCK_NUM_LIMIT3 {
        n_groups = BZP_NGROUPS_NUM_3;
    } else {
        n_groups = BZP_NGROUPS_NUM_4;
    }
    n_groups
}
