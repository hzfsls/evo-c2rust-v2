type RapidlzCompressFunc = fn(strm_ctx: *mut RapidlzStreamCtx, src: *const c_char, dest: *mut c_char, src_size: c_int, dest_size: c_int) -> c_int;
