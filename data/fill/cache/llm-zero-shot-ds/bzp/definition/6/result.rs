#[repr(C)]
pub struct BzpBwtInfo {
    pub sort_block: *mut i32,
    pub idx: *mut i32,
    pub is_start_pos: *mut i32,
    pub block: *mut u8,
    pub block_crc: u32,
    pub combined_crc: u32,
    pub n_block_max: i32,
    pub block_id: i32,
    pub n_block: i32,
    pub ori_ptr: i32,
    pub in_use: [bool; BZP_ASCII_SIZE],
}
