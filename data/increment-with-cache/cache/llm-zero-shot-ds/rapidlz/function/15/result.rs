use std::ptr;

static RAPIDLZ_MAX_4BIT_VALUE: u32 = 0xF;
static RAPIDLZ_COPY_PROTECT_SIZE: usize = 8;
static RAPIDLZ_LAST_LITERALS: usize = 5;

#[inline(always)]
fn rapidlz_copy_16_byte(dest: *mut u8, src: *const u8) {
    unsafe {
        ptr::copy_nonoverlapping(src, dest, 16);
    }
}

#[inline(always)]
fn rapidlz_wild_copy_16(src: *const u8, dest: *mut u8, end: *mut u8) {
    unsafe {
        let mut d = dest;
        let mut s = src;
        while d < end {
            ptr::copy_nonoverlapping(s, d, 1);
            d = d.offset(1);
            s = s.offset(1);
        }
    }
}

#[inline(always)]
fn rapidlz_read_le_16_bit(src: *const u8) -> u16 {
    unsafe {
        ptr::read_unaligned(src as *const u16).to_le()
    }
}

#[inline(always)]
fn rapidlz_copy_match_fast(dest: *mut u8, match_src: *const u8, offset: u16, len: u32) {
    unsafe {
        if offset >= 8 {
            ptr::copy_nonoverlapping(match_src, dest, len as usize);
        } else {
            let mut d = dest;
            let mut m = match_src;
            let end = dest.offset(len as isize);
            while d < end {
                ptr::copy_nonoverlapping(m, d, 1);
                d = d.offset(1);
                m = m.offset(1);
            }
        }
    }
}

#[inline(always)]
fn rapidlz_safe_copy_match_fast(dest: *mut u8, match_src: *const u8, dest_end: *const u8, offset: u16, len: u32) {
    unsafe {
        if offset >= 8 {
            let copy_len = len.min((dest_end as usize - dest as usize) as u32) as usize;
            ptr::copy_nonoverlapping(match_src, dest, copy_len);
        } else {
            let mut d = dest;
            let mut m = match_src;
            let end = dest.offset(len as isize);
            while d < end && d < dest_end {
                ptr::copy_nonoverlapping(m, d, 1);
                d = d.offset(1);
                m = m.offset(1);
            }
        }
    }
}

#[inline(always)]
fn rapidlz_fast_safe_copy_by_bytes(dest: *mut u8, src: *const u8, len: u32) {
    unsafe {
        let mut d = dest;
        let mut s = src;
        let end = dest.offset(len as isize);
        while d < end {
            ptr::copy_nonoverlapping(s, d, 1);
            d = d.offset(1);
            s = s.offset(1);
        }
    }
}

fn rapidlz_dec_with_prefix_dict(
    src: *const u8,
    dest: *mut u8,
    src_size: i32,
    out_buffer_size: i32,
    _dict_start: *const u8,
    _dict_size: i32,
) -> i32 {
    let cur_src = src;
    let src_end = unsafe { cur_src.offset(src_size as isize) };
    let mut cur_dest = dest;
    let dest_end = unsafe { cur_dest.offset(out_buffer_size as isize) };

    let src_end_fast = unsafe { src_end.offset(-(RAPIDLZ_COPY_PROTECT_SIZE as isize)) };
    let dest_end_fast = unsafe { dest_end.offset(-(RAPIDLZ_COPY_PROTECT_SIZE as isize)) };

    let mut tmp = 0;
    
    loop {
        let token = unsafe { *cur_src };
        let mut cur_src = unsafe { cur_src.offset(1) };

        let mut len = (token >> 4) as u32;
        if len < RAPIDLZ_MAX_4BIT_VALUE {
            if unsafe { cur_src.offset(len as isize) <= src_end_fast && cur_dest.offset(len as isize) <= dest_end_fast } {
                rapidlz_copy_16_byte(cur_dest, cur_src);
                cur_src = unsafe { cur_src.offset(len as isize) };
                cur_dest = unsafe { cur_dest.offset(len as isize) };
            } else {
                let left_src_size = unsafe { src_end.offset_from(cur_src) } as usize;
                // Implement RAPIDLZ_SAFE_COPY_TILL_END logic here
                let copy_len = len.min(left_src_size as u32).min(unsafe { dest_end.offset_from(cur_dest) } as usize as u32);
                unsafe {
                    ptr::copy_nonoverlapping(cur_src, cur_dest, copy_len as usize);
                }
                cur_src = unsafe { cur_src.offset(copy_len as isize) };
                cur_dest = unsafe { cur_dest.offset(copy_len as isize) };
            }
        } else {
            // Implement RAPIDLZ_READ_OPTIONAL_LENGTH logic here
            if unsafe { cur_src >= src_end } {
                break;
            }
            let byte = unsafe { *cur_src };
            cur_src = unsafe { cur_src.offset(1) };
            len += byte as u32;
            if byte == 0xFF {
                if unsafe { cur_src >= src_end } {
                    break;
                }
                let byte = unsafe { *cur_src };
                cur_src = unsafe { cur_src.offset(1) };
                len += byte as u32;
            }

            if unsafe { cur_src.offset(len as isize) <= src_end_fast && cur_dest.offset(len as isize) <= dest_end_fast } {
                rapidlz_wild_copy_16(cur_src, cur_dest, unsafe { cur_dest.offset(len as isize) });
                cur_src = unsafe { cur_src.offset(len as isize) };
                cur_dest = unsafe { cur_dest.offset(len as isize) };
            } else {
                let left_src_size = unsafe { src_end.offset_from(cur_src) } as usize;
                // Implement RAPIDLZ_SAFE_COPY_TILL_END logic here
                let copy_len = len.min(left_src_size as u32).min(unsafe { dest_end.offset_from(cur_dest) } as usize as u32);
                unsafe {
                    ptr::copy_nonoverlapping(cur_src, cur_dest, copy_len as usize);
                }
                cur_src = unsafe { cur_src.offset(copy_len as isize) };
                cur_dest = unsafe { cur_dest.offset(copy_len as isize) };
            }
        }

        if unsafe { cur_src.offset(2) > src_end } {
            break;
        }
        let offset = rapidlz_read_le_16_bit(cur_src);
        cur_src = unsafe { cur_src.offset(2) };
        let match_src = unsafe { cur_dest.offset(-(offset as isize)) };

        len = (token & RAPIDLZ_MAX_4BIT_VALUE as u8) as u32;
        if len < RAPIDLZ_MAX_4BIT_VALUE {
            len += 4;
        } else {
            // Implement RAPIDLZ_READ_OPTIONAL_LENGTH logic here
            if unsafe { cur_src >= src_end } {
                break;
            }
            let byte = unsafe { *cur_src };
            cur_src = unsafe { cur_src.offset(1) };
            len += byte as u32;
            if byte == 0xFF {
                if unsafe { cur_src >= src_end } {
                    break;
                }
                let byte = unsafe { *cur_src };
                cur_src = unsafe { cur_src.offset(1) };
                len += byte as u32;
            }
            len += 4;
        }

        if unsafe { cur_dest.offset(len as isize) <= dest_end_fast.offset(-(RAPIDLZ_LAST_LITERALS as isize)) } {
            rapidlz_copy_match_fast(cur_dest, match_src, offset, len);
            cur_dest = unsafe { cur_dest.offset(len as isize) };
        } else {
            if len < 1024 {
                rapidlz_fast_safe_copy_by_bytes(cur_dest, match_src, len);
            } else {
                rapidlz_safe_copy_match_fast(cur_dest, match_src, dest_end, offset, len);
                cur_dest = unsafe { cur_dest.offset(len as isize) };
            }
        }
    }

    unsafe { cur_dest.offset_from(dest) as i32 }
}
