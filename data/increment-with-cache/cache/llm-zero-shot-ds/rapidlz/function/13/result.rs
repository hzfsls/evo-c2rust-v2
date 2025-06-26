use std::cmp::{min, max};

pub fn rapidlz_strm_ctx_norm(strm_ctx: &mut RapidlzStreamCtx, src: *mut u8, src_size: i32, dict_end: *mut u8) {
    rapidlz_dict_size_norm(strm_ctx, src, dict_end);
    
    strm_ctx.acceleration = max(
        RAPIDLZ_STREAM_ACCELERAT_MIN,
        min(RAPIDLZ_STREAM_ACCELERAT_MAX, strm_ctx.acceleration),
    );
    
    if strm_ctx.current_offset.wrapping_add(src_size as u32) > RAPIDLZ_PTR_DIFF_MAX_32 {
        let delta = strm_ctx.current_offset - RAPIDLZ_MAX_DICT_SIZE;
        
        for i in 0..RAPIDLZ_STREAM_HASH_SIZE {
            if strm_ctx.hash_table[i] < delta {
                strm_ctx.hash_table[i] = 0;
            } else {
                strm_ctx.hash_table[i] -= delta;
            }
        }
        
        strm_ctx.current_offset = RAPIDLZ_MAX_DICT_SIZE;
    }
    
    let src_end = unsafe { src.offset(src_size as isize) };
    
    if src_end > strm_ctx.dict as *mut u8 && src_end < dict_end {
        let dict_size = unsafe { dict_end.offset_from(src_end) } as u32;
        
        strm_ctx.dict_size = if dict_size > RAPIDLZ_MAX_DICT_SIZE {
            RAPIDLZ_MAX_DICT_SIZE
        } else if dict_size < RAPIDLZ_STREAM_HASH_BYTES {
            0
        } else {
            dict_size
        };
        
        strm_ctx.dict = unsafe { dict_end.offset(-(strm_ctx.dict_size as isize)) as *mut u8 };
    }
}
