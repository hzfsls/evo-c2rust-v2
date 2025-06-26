use std::ptr;

static RAPIDLZ_LAST_LITERALS: usize = 5;
static RAPIDLZ_MIN_COMPRESS_SIZE: usize = 13;
static RAPIDLZ_MIN_MATCH: usize = 4;
static RAPIDLZ_MAX_OFFSET: usize = 65535;
static RAPIDLZ_MAX_BYTE_VALUE: usize = 255;

#[derive(Debug)]
struct RapidlzCCtx {
    hash_table: Vec<u16>,
    hash_type: u8,
    hash_bits: u8,
    step: u32,
    buffer_limit: u8,
}

fn rapidlz_compress_process(
    dst: &mut [u8],
    src: &[u8],
    c_ctx: &mut RapidlzCCtx,
) -> usize {
    let mut step = 1;
    let src_begin = src.as_ptr();
    let src_end = unsafe { src_begin.add(src.len()) };
    let mut src_curr = unsafe { src_begin.add(1) };
    let src_anchor = src_begin;
    let match_limit = unsafe { src_end.sub(RAPIDLZ_LAST_LITERALS) };
    let src_limit = unsafe { src_end.sub(RAPIDLZ_MIN_COMPRESS_SIZE) };
    let dst_begin = dst.as_mut_ptr();
    let dst_end = unsafe { dst_begin.add(dst.len()) };
    let mut dst_curr = dst_begin;
    let hash_type = c_ctx.hash_type;
    let hash_bits = c_ctx.hash_bits;
    let mut search_match_nb = c_ctx.step << RAPIDLZ_STEP_FORWARD_BASE;
    let mut search_match_nb_tmp = search_match_nb;
    let buffer_limit = c_ctx.buffer_limit;

    while likely(src_curr <= src_limit) {
        loop {
            let hash_value = rapidlz_calc_hash_value(src_curr, hash_type, hash_bits);
            let match_begin = unsafe { src_begin.add(rapidlz_get_pos_on_table(hash_value, &c_ctx.hash_table, hash_type)) };
            rapidlz_put_pos_on_table(
                unsafe { src_curr.offset_from(src_begin) } as u32,
                hash_value,
                &mut c_ctx.hash_table,
                hash_type,
            );
            if likely(unsafe { ptr::read_unaligned(src_curr as *const u32) == ptr::read_unaligned(match_begin as *const u32) })
                && likely(unsafe { src_curr.offset_from(match_begin) } as usize <= RAPIDLZ_MAX_OFFSET)
            {
                break;
            }
            src_curr = unsafe { src_curr.add(step) };
            step = search_match_nb_tmp >> RAPIDLZ_STEP_FORWARD_BASE;
            search_match_nb_tmp += 1;
            if src_curr > src_limit {
                dst_curr = rapidlz_store_last_literals(
                    dst_curr,
                    dst_end,
                    src_anchor,
                    unsafe { src_end.offset_from(src_anchor) } as usize,
                    buffer_limit,
                );
                if dst_curr.is_null() {
                    return 0;
                }
                return unsafe { dst_curr.offset_from(dst_begin) } as usize;
            }
        }
        step = 1;
        search_match_nb_tmp = search_match_nb;
        let src_curr_match_end = rapidlz_compress_expand_backward(
            match_limit,
            unsafe { match_begin.add(RAPIDLZ_MIN_MATCH) },
            unsafe { src_curr.add(RAPIDLZ_MIN_MATCH) },
        );
        rapidlz_expand_forward(src_begin, match_begin, src_curr, src_anchor);
        let match_length = unsafe { src_curr_match_end.offset_from(src_curr) } as usize - RAPIDLZ_MIN_MATCH;
        let offset = unsafe { src_curr.offset_from(match_begin) } as u16;
        let literal_length = unsafe { src_curr.offset_from(src_anchor) } as usize;
        if buffer_limit != 0 {
            let write_size = literal_length + 8 + (literal_length + match_length / RAPIDLZ_MAX_BYTE_VALUE);
            if unlikely(unsafe { dst_curr.add(write_size) } > dst_end) {
                // RAPIDLZ_LOG(RAPIDLZ_DST_SIZE_SMALL, "dstEnd - dstCur:%zu writeSize:%u\n", dst_end - dst_curr, write_size);
                return 0;
            }
        }
        dst_curr = rapidlz_store_sequence(
            dst_curr,
            src_anchor,
            literal_length,
            match_length,
            offset,
        );
        src_curr = src_curr_match_end;
        src_anchor = src_curr;
        let hash_value = rapidlz_calc_hash_value(unsafe { src_curr.sub(2) }, hash_type, hash_bits);
        rapidlz_put_pos_on_table(
            unsafe { src_curr.sub(2).offset_from(src_begin) } as u32,
            hash_value,
            &mut c_ctx.hash_table,
            hash_type,
        );
        if unlikely(src_curr > src_limit) {
            break;
        }
        let hash_value = rapidlz_calc_hash_value(src_curr, hash_type, hash_bits);
        let match_begin = unsafe { src_begin.add(rapidlz_get_pos_on_table(hash_value, &c_ctx.hash_table, hash_type)) };
        rapidlz_put_pos_on_table(
            unsafe { src_curr.offset_from(src_begin) } as u32,
            hash_value,
            &mut c_ctx.hash_table,
            hash_type,
        );
        if unlikely(unsafe { ptr::read_unaligned(src_curr as *const u32) != ptr::read_unaligned(match_begin as *const u32) })
            || unlikely(unsafe { src_curr.offset_from(match_begin) } as usize > RAPIDLZ_MAX_OFFSET)
        {
            src_curr = unsafe { src_curr.add(1) };
            continue;
        }
        let src_curr_match_end = rapidlz_compress_expand_backward(
            match_limit,
            unsafe { match_begin.add(RAPIDLZ_MIN_MATCH) },
            unsafe { src_curr.add(RAPIDLZ_MIN_MATCH) },
        );
        let match_length = unsafe { src_curr_match_end.offset_from(src_curr) } as usize - RAPIDLZ_MIN_MATCH;
        let offset = unsafe { src_curr.offset_from(match_begin) } as u16;
        if buffer_limit != 0 {
            let write_size = 8 + match_length / RAPIDLZ_MAX_BYTE_VALUE;
            if unlikely(unsafe { dst_curr.add(write_size) } > dst_end) {
                // RAPIDLZ_LOG(RAPIDLZ_DST_SIZE_SMALL, "dstEnd - dstCur:%zu writeSize:%u\n", dst_end - dst_curr, write_size);
                return 0;
            }
        }
        unsafe { *dst_curr = 0 };
        dst_curr = rapidlz_store_off_match(unsafe { dst_curr.add(1) }, dst_curr, match_length, offset);
        src_curr = src_curr_match_end;
        src_anchor = src_curr;
        let hash_value = rapidlz_calc_hash_value(unsafe { src_curr.sub(2) }, hash_type, hash_bits);
        rapidlz_put_pos_on_table(
            unsafe { src_curr.sub(2).offset_from(src_begin) } as u32,
            hash_value,
            &mut c_ctx.hash_table,
            hash_type,
        );
    }
    if src_anchor < src_end {
        dst_curr = rapidlz_store_last_literals(
            dst_curr,
            dst_end,
            src_anchor,
            unsafe { src_end.offset_from(src_anchor) } as usize,
            buffer_limit,
        );
        if dst_curr.is_null() {
            return 0;
        }
    }
    unsafe { dst_curr.offset_from(dst_begin) } as usize
}

// Helper functions (assuming they exist elsewhere in the code)
fn likely(b: bool) -> bool { b }
fn unlikely(b: bool) -> bool { b }
fn rapidlz_calc_hash_value(ptr: *const u8, hash_type: u8, hash_bits: u8) -> u32 { /* ... */ 0 }
fn rapidlz_get_pos_on_table(hash_value: u32, hash_table: &[u16], hash_type: u8) -> usize { /* ... */ 0 }
fn rapidlz_put_pos_on_table(pos: u32, hash_value: u32, hash_table: &mut [u16], hash_type: u8) { /* ... */ }
fn rapidlz_store_last_literals(dst: *mut u8, dst_end: *mut u8, src: *const u8, len: usize, buffer_limit: u8) -> *mut u8 { /* ... */ dst }
fn rapidlz_compress_expand_backward(match_limit: *const u8, a: *const u8, b: *const u8) -> *const u8 { /* ... */ b }
fn rapidlz_expand_forward(src_begin: *const u8, match_begin: *const u8, src_curr: *const u8, src_anchor: *const u8) { /* ... */ }
fn rapidlz_store_sequence(dst: *mut u8, src: *const u8, literal_length: usize, match_length: usize, offset: u16) -> *mut u8 { /* ... */ dst }
fn rapidlz_store_off_match(dst: *mut u8, _: *mut u8, match_length: usize, offset: u16) -> *mut u8 { /* ... */ dst }
