pub struct CmptlzCompParam {
    pub level: i32,
    pub dict_size: u32,
    pub lit_ctx: i32,
    pub lit_pos: i32,
    pub pos_bits: i32,
    pub fast_bytes: i32,
    pub num_threads: i32,
    pub prot_data: *mut u8,
    pub prot_size: usize,
    pub mem_hook: *mut CmptLzMemHook,
}
