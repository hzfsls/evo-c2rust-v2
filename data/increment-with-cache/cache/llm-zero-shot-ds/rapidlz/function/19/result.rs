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

    let mut token;
    let mut temp = 0;
    let mut offset: u16;
    let mut lit_len: u32;
    let mut match_len: u32;
    let mut match_src: *mut u8;
    let src_end = unsafe { src.add(src_size) };
    let mut src_curr = src;
    let src_end_fast = unsafe { src_end.sub(RAPIDLZ_COPY_PROTECT_SIZE) };
    let dst_end = unsafe { dst.add(dst_size) };
    let mut dst_curr = dst;
    let dst_end_fast = unsafe { dst_end.sub(RAPIDLZ_COPY_PROTECT_SIZE) };

    while src_curr < src_end {
        token = unsafe { *src_curr };
        src_curr = unsafe { src_curr.add(1) };
        lit_len = (token >> 4) as u32;

        if lit_len < RAPIDLZ_MAX_4BIT_VALUE {
            if unsafe { src_curr.add(lit_len as usize) <= src_end_fast }
                && unsafe { dst_curr.add(lit_len as usize) <= dst_end_fast }
            {
                rapidlz_copy_16_byte(dst_curr, src_curr);
                dst_curr = unsafe { dst_curr.add(lit_len as usize) };
                src_curr = unsafe { src_curr.add(lit_len as usize) };
                goto READ_MATCH;
            }
        } else {
            rapidlz_read_optional_length(&mut lit_len, &mut src_curr, src_end, &mut temp);
            if unsafe { src_curr.add(lit_len as usize) <= src_end_fast }
                && unsafe { dst_curr.add(lit_len as usize) <= dst_end_fast }
            {
                rapidlz_wild_copy_16(src_curr, dst_curr, unsafe { dst_curr.add(lit_len as usize) });
                dst_curr = unsafe { dst_curr.add(lit_len as usize) };
                src_curr = unsafe { src_curr.add(lit_len as usize) };
                goto READ_MATCH;
            }
        }

        let left_src_size = unsafe { src_end.offset_from(src_curr) } as usize;
        if lit_len > left_src_size
            || unsafe {
                ptr::copy_nonoverlapping(
                    src_curr,
                    dst_curr,
                    lit_len.min((dst_end as usize - dst_curr as usize) as usize),
                );
                lit_len > (dst_end as usize - dst_curr as usize) as usize
            }
        {
            // RAPIDLZ_LOG(RAPIDLZ_DST_SIZE_SMALL, "litLen:%u dstEnd - dst:%zu\n", lit_len, left_src_size);
            return 0;
        }

        dst_curr = unsafe { dst_curr.add(lit_len as usize) };
        src_curr = unsafe { src_curr.add(lit_len as usize) };

        if left_src_size == lit_len as usize {
            return unsafe { dst_curr.offset_from(dst) } as usize;
        }

        READ_MATCH:
        if unsafe { src_curr > src_end.sub(2) } {
            // RAPIDLZ_LOG(RAPIDLZ_FORMAT_INVALID, "rapidlz format invalid\n");
            return 0;
        }
        offset = rapidlz_read_le_16_bit(&mut src_curr);
        src_curr = unsafe { src_curr.add(2) };
        match_src = unsafe { dst_curr.sub(offset as usize) };
        if (match_src as *const u8) < dst as *const u8 {
            // RAPIDLZ_LOG(RAPIDLZ_FORMAT_INVALID, "rapidlz format invalid\n");
            return 0;
        }

        match_len = (token & RAPIDLZ_MAX_4BIT_VALUE) as u32 + RAPIDLZ_MIN_MATCH;
        if match_len == RAPIDLZ_MAX_4BIT_MATCH {
            rapidlz_read_optional_length(&mut match_len, &mut src_curr, src_end, &mut temp);
        }

        if unsafe { dst_curr.add(match_len as usize) <= dst_end_fast } {
            rapidlz_copy_match_fast(dst_curr, match_src, offset, match_len);
            dst_curr = unsafe { dst_curr.add(match_len as usize) };
        } else {
            if unsafe { dst_curr.add(match_len as usize) > dst_end } {
                // RAPIDLZ_LOG(RAPIDLZ_DST_SIZE_SMALL, "dstEnd - dstCurr:%zu matchLen:%u\n", dst_end - dst_curr, match_len);
                return 0;
            }

            safe_copy_match(dst_curr, match_src, match_len);
        }
    }

    unsafe { dst_curr.offset_from(dst) } as usize
}

// Helper functions (assuming they're defined elsewhere)
fn rapidlz_copy_16_byte(dst: *mut u8, src: *const u8) {
    // Implementation of RapidlzCopy16Byte
    unsafe {
        ptr::copy_nonoverlapping(src, dst, 16);
    }
}

fn rapidlz_wild_copy_16(src: *const u8, dst: *mut u8, end: *mut u8) {
    // Implementation of RapidlzWildCopy16
    let mut len = unsafe { end.offset_from(dst) } as usize;
    while len >= 16 {
        rapidlz_copy_16_byte(dst, src);
        len -= 16;
        unsafe {
            dst = dst.add(16);
            src = src.add(16);
        }
    }
    if len > 0 {
        unsafe {
            ptr::copy_nonoverlapping(src, dst, len);
        }
    }
}

fn rapidlz_read_optional_length(len: &mut u32, src: &mut *const u8, end: *const u8, temp: &mut u8) {
    // Implementation of RAPIDLZ_READ_OPTIONAL_LENGTH
    while *src < end && **src == 0xFF {
        *len += 0xFF;
        *src = unsafe { src.add(1) };
    }
    if *src < end {
        *len += **src as u32;
        *src = unsafe { src.add(1) };
    }
}

fn rapidlz_read_le_16_bit(src: &mut *const u8) -> u16 {
    // Implementation of RapidlzReadLE16Bit
    let bytes = unsafe { [*src.add(0), *src.add(1)] };
    u16::from_le_bytes(bytes)
}

fn rapidlz_copy_match_fast(dst: *mut u8, src: *const u8, offset: u16, len: u32) {
    // Implementation of RapidlzCopyMatchFast
    if offset >= 16 {
        unsafe {
            ptr::copy_nonoverlapping(src, dst, len as usize);
        }
    } else {
        // Handle overlapping copies for small offsets
        for i in 0..len as usize {
            unsafe {
                *dst.add(i) = *src.add(i % offset as usize);
            }
        }
    }
}

fn safe_copy_match(dst: *mut u8, src: *const u8, len: u32) {
    // Implementation of SAFE_COPY_MATCH
    for i in 0..len as usize {
        unsafe {
            *dst.add(i) = *src.add(i);
        }
    }
}

// Constants (assuming they're defined elsewhere)
const RAPIDLZ_COPY_PROTECT_SIZE: usize = 32;
const RAPIDLZ_MAX_4BIT_VALUE: u32 = 15;
const RAPIDLZ_MIN_MATCH: u32 = 4;
const RAPIDLZ_MAX_4BIT_MATCH: u32 = 19;
