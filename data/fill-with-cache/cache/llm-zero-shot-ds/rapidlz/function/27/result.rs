use std::ptr;

pub fn rapidlz_decompress(
    src: *const u8,
    dst: *mut u8,
    src_size: usize,
    dst_size: usize,
) -> usize {
    if src.is_null() || dst.is_null() || src_size == 0 || dst_size == 0 {
        // RAPIDLZ_LOG(RAPIDLZ_INPUT_INVALID, "input invalid\n");
        return 0;
    }

    let src_end = unsafe { src.add(src_size) };
    let mut src_curr = src;
    let src_end_fast = unsafe { src_end.sub(RAPIDLZ_COPY_PROTECT_SIZE) };

    let dst_end = unsafe { dst.add(dst_size) };
    let mut dst_curr = dst;
    let dst_end_fast = unsafe { dst_end.sub(RAPIDLZ_COPY_PROTECT_SIZE) };

    while src_curr < src_end {
        let token = unsafe { *src_curr };
        src_curr = unsafe { src_curr.add(1) };

        let mut lit_len = (token >> 4) as usize;
        if lit_len < RAPIDLZ_MAX_4BIT_VALUE as usize {
            if unsafe { src_curr.add(lit_len) <= src_end_fast && dst_curr.add(lit_len) <= dst_end_fast } {
                rapidlz_copy_16_byte(dst_curr, src_curr);
                dst_curr = unsafe { dst_curr.add(lit_len) };
                src_curr = unsafe { src_curr.add(lit_len) };
                // goto READ_MATCH
            } else {
                let left_src_size = unsafe { src_end.offset_from(src_curr) } as usize;
                if lit_len > left_src_size {
                    // RAPIDLZ_LOG(RAPIDLZ_DST_SIZE_SMALL, "litLen:%u dstEnd - dst:%zu\n", lit_len, left_src_size);
                    return 0;
                }
                unsafe {
                    ptr::copy_nonoverlapping(src_curr, dst_curr, lit_len);
                }
                dst_curr = unsafe { dst_curr.add(lit_len) };
                src_curr = unsafe { src_curr.add(lit_len) };
                if left_src_size == lit_len {
                    return unsafe { dst_curr.offset_from(dst) } as usize;
                }
                // goto READ_MATCH
            }
        } else {
            let mut temp = 0;
            rapidlz_read_optional_length(&mut lit_len, &mut src_curr, src_end, &mut temp);
            if unsafe { src_curr.add(lit_len) <= src_end_fast && dst_curr.add(lit_len) <= dst_end_fast } {
                rapidlz_wild_copy_16(src_curr, dst_curr, unsafe { dst_curr.add(lit_len) });
                dst_curr = unsafe { dst_curr.add(lit_len) };
                src_curr = unsafe { src_curr.add(lit_len) };
                // goto READ_MATCH
            } else {
                let left_src_size = unsafe { src_end.offset_from(src_curr) } as usize;
                if lit_len > left_src_size {
                    // RAPIDLZ_LOG(RAPIDLZ_DST_SIZE_SMALL, "litLen:%u dstEnd - dst:%zu\n", lit_len, left_src_size);
                    return 0;
                }
                unsafe {
                    ptr::copy_nonoverlapping(src_curr, dst_curr, lit_len);
                }
                dst_curr = unsafe { dst_curr.add(lit_len) };
                src_curr = unsafe { src_curr.add(lit_len) };
                if left_src_size == lit_len {
                    return unsafe { dst_curr.offset_from(dst) } as usize;
                }
                // goto READ_MATCH
            }
        }

        // READ_MATCH:
        if unsafe { src_curr > src_end.sub(2) } {
            // RAPIDLZ_LOG(RAPIDLZ_FORMAT_INVALID, "rapidlz format invalid\n");
            return 0;
        }

        let offset = rapidlz_read_le_16_bit(&mut src_curr) as usize;
        let match_src = unsafe { dst_curr.offset(-(offset as isize)) };
        if match_src < dst {
            // RAPIDLZ_LOG(RAPIDLZ_FORMAT_INVALID, "rapidlz format invalid\n");
            return 0;
        }

        let mut match_len = (token & RAPIDLZ_MAX_4BIT_VALUE) as usize + RAPIDLZ_MIN_MATCH as usize;
        if match_len == RAPIDLZ_MAX_4BIT_MATCH as usize {
            let mut temp = 0;
            rapidlz_read_optional_length(&mut match_len, &mut src_curr, src_end, &mut temp);
        }

        if unsafe { dst_curr.add(match_len) <= dst_end_fast } {
            rapidlz_copy_match_fast(dst_curr, match_src, offset, match_len);
            dst_curr = unsafe { dst_curr.add(match_len) };
        } else {
            if unsafe { dst_curr.add(match_len) > dst_end } {
                // RAPIDLZ_LOG(RAPIDLZ_DST_SIZE_SMALL, "dstEnd - dstCurr:%zu matchLen:%u\n", 
                //     unsafe { dst_end.offset_from(dst_curr) } as usize, match_len);
                return 0;
            }
            safe_copy_match(dst_curr, match_src, match_len);
        }
    }

    unsafe { dst_curr.offset_from(dst) } as usize
}

// Constants (assuming these are defined elsewhere)
const RAPIDLZ_COPY_PROTECT_SIZE: usize = 16;
const RAPIDLZ_MAX_4BIT_VALUE: u8 = 15;
const RAPIDLZ_MIN_MATCH: u8 = 4;
const RAPIDLZ_MAX_4BIT_MATCH: u8 = 19;

// Helper functions (assuming these are implemented elsewhere)
fn rapidlz_copy_16_byte(dst: *mut u8, src: *const u8) {
    unsafe {
        ptr::copy_nonoverlapping(src, dst, 16);
    }
}

fn rapidlz_wild_copy_16(src: *const u8, dst: *mut u8, end: *mut u8) {
    let mut len = unsafe { end.offset_from(dst) } as usize;
    while len >= 16 {
        rapidlz_copy_16_byte(dst, src);
        len -= 16;
    }
    if len > 0 {
        unsafe {
            ptr::copy_nonoverlapping(src, dst, len);
        }
    }
}

fn rapidlz_read_optional_length(
    len: &mut usize,
    src: &mut *const u8,
    src_end: *const u8,
    temp: &mut u8,
) {
    unsafe {
        *temp = *(*src);
        *src = (*src).add(1);
        *len += *temp as usize;
        if *temp == 0xff {
            while *src < src_end {
                *temp = *(*src);
                *src = (*src).add(1);
                *len += *temp as usize;
                if *temp != 0xff {
                    break;
                }
            }
        }
    }
}

fn rapidlz_read_le_16_bit(src: &mut *const u8) -> u16 {
    unsafe {
        let bytes = [*(*src), *(*src).add(1)];
        *src = (*src).add(2);
        u16::from_le_bytes(bytes)
    }
}

fn rapidlz_copy_match_fast(dst: *mut u8, src: *const u8, offset: usize, len: usize) {
    if offset >= 16 {
        unsafe {
            ptr::copy_nonoverlapping(src, dst, len);
        }
    } else {
        // Handle overlapping copies for small offsets
        unsafe {
            for i in 0..len {
                *dst.add(i) = *src.add(i % offset);
            }
        }
    }
}

fn safe_copy_match(dst: *mut u8, src: *const u8, len: usize) {
    unsafe {
        ptr::copy_nonoverlapping(src, dst, len);
    }
}
