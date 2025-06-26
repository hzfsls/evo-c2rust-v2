pub struct TagRapidlzStreamCtx {
    hash_table: [u32; RAPIDLZ_STREAM_HASH_SIZE],
    dict: *const u8,
    dict_size: u32,
    current_offset: u32,
    acceleration: i32,
    strm_ctx_specific: *mut RapidlzStreamCtx,
}
