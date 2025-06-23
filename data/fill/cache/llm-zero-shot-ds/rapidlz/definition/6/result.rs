pub struct RapidlzCCtx {
    hash_table: *mut u8,
    hash_type: u8,
    hash_bits: u8,
    step: u8,
    buffer_limit: u8,
}
