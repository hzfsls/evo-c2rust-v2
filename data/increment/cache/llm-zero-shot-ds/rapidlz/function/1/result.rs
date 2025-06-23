use std::ptr;

pub fn rapidlz_dict_size_norm(strm_ctx: &mut RapidlzStreamCtx, src: *mut u8, dict_end: *mut *mut u8) {
    if strm_ctx.dict_size < RAPIDLZ_STREAM_HASH_BYTES {
        strm_ctx.dict_size = 0;
        strm_ctx.dict = src;
        *dict_end = src;
    }
    if strm_ctx.dict_size > RAPIDLZ_MAX_DICT_SIZE {
        unsafe {
            strm_ctx.dict = ptr::sub(dict_end, RAPIDLZ_MAX_DICT_SIZE);
        }
        strm_ctx.dict_size = RAPIDLZ_MAX_DICT_SIZE;
    }
}
