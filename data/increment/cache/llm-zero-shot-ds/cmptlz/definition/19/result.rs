pub struct TagCmptMatchFinder {
    src_start: *const u8,
    src_len: usize,

    hash_root_table: [u32; 256],
    mf_start: u32,
    nice_len: u32,
    read_ahead: u32,
    read_pos: u32,
    cycle_pos: u32,
    cycle_size: u32,
    offset: u32,
    hash: *mut u32,
    son: *mut u32,
    depth: u32,
    hash_count: u32,
    sons_count: u32,
    hash_mask: u32,
}
