pub struct RapidlzCCtx {
    pub hash_table: *mut u8,
    pub hash_type: u8,
    pub hash_bits: u8,
    pub step: u8,
    pub buffer_limit: u8,
}
