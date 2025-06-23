use std::mem;

// Assuming RapidlzIsLE, RapidlzCountTailZero64, RapidlzCountLeadZero64, RAPIDLZ_READ64BIT, 
// RAPIDLZ_READ32BIT, RAPIDLZ_READ16BIT are defined elsewhere

#[inline]
fn rapidlz_compress_expand_backward(
    match_limit: *const u8,
    match_ptr: *const u8,
    src_curr: *const u8,
) -> *const u8 {
    let mut xor_val: u64;
    let loop_end = unsafe { match_limit.offset(-7) };
    let mut src_curr_match_end = src_curr;
    let mut match_begin = match_ptr;

    unsafe {
        while src_curr_match_end < loop_end {
            xor_val = RAPIDLZ_READ64BIT(match_begin) ^ RAPIDLZ_READ64BIT(src_curr_match_end);
            if xor_val == 0 {
                src_curr_match_end = src_curr_match_end.offset(mem::size_of::<u64>() as isize);
                match_begin = match_begin.offset(mem::size_of::<u64>() as isize);
                continue;
            }
            let offset = if RapidlzIsLE() {
                RapidlzCountTailZero64(xor_val) >> 3
            } else {
                RapidlzCountLeadZero64(xor_val) >> 3
            };
            src_curr_match_end = src_curr_match_end.offset(offset as isize);
            return src_curr_match_end;
        }

        if src_curr_match_end.offset(3) < match_limit
            && RAPIDLZ_READ32BIT(src_curr_match_end) == RAPIDLZ_READ32BIT(match_begin)
        {
            src_curr_match_end = src_curr_match_end.offset(mem::size_of::<u32>() as isize);
            match_begin = match_begin.offset(mem::size_of::<u32>() as isize);
        }

        if src_curr_match_end.offset(1) < match_limit
            && RAPIDLZ_READ16BIT(src_curr_match_end) == RAPIDLZ_READ16BIT(match_begin)
        {
            src_curr_match_end = src_curr_match_end.offset(mem::size_of::<u16>() as isize);
            match_begin = match_begin.offset(mem::size_of::<u16>() as isize);
        }

        if src_curr_match_end < match_limit && *src_curr_match_end == *match_begin {
            src_curr_match_end = src_curr_match_end.offset(1);
        }
    }
    src_curr_match_end
}
