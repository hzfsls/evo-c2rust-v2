use std::ptr;

static RAPIDLZ_COPY_PROTECT_SIZE: usize = 16;
static RAPIDLZ_MAX_4BIT_VALUE: u32 = 0xF;
static RAPIDLZ_LAST_LITERALS: usize = 5;

#[inline(always)]
fn rapidlz_likely(condition: bool) -> bool {
    condition
}

#[inline(always)]
fn rapidlz_copy_16_byte(dest: *mut u8, src: *const u8) {
    unsafe {
        ptr::copy_nonoverlapping(src, dest, 16);
    }
}

#[inline(always)]
fn rapidlz_wild_copy_16(src: *const u8, dest: *mut u8, end: *mut u8) {
    unsafe {
        let mut src = src;
        let mut dest = dest;
        while dest < end {
            ptr::copy_nonoverlapping(src, dest, 1);
            src = src.add(1);
            dest = dest.add(1);
        }
    }
}

#[inline(always)]
fn rapidlz_read_le_16_bit(src: *const u8) -> u16 {
    unsafe {
        let bytes = [*src, *src.add(1)];
        u16::from_le_bytes(bytes)
    }
}

#[inline(always)]
fn rapidlz_copy_match_fast(dest: *mut u8, match_src: *const u8, offset: u16, len: u32) {
    unsafe {
        ptr::copy_nonoverlapping(match_src, dest, len as usize);
    }
}

#[inline(always)]
fn rapidlz_safe_copy_match_fast(dest: *mut u8, match_src: *const u8, dest_end: *mut u8, offset: u16, len: u32) {
    unsafe {
        let copy_len = (dest_end as usize - dest as usize).min(len as usize);
        ptr::copy_nonoverlapping(match_src, dest, copy_len);
    }
}

#[inline(always)]
fn rapidlz_fast_safe_copy_by_bytes(dest: *mut u8, src: *const u8, len: usize) {
    unsafe {
        ptr::copy_nonoverlapping(src, dest, len);
    }
}

fn rapidlz_dec_with_external_dict(
    src: *const u8,
    dest: *mut u8,
    src_size: i32,
    out_buffer_size: i32,
    dict_start: *const u8,
    dict_size: i32,
) -> i32 {
    let mut cur_src = src;
    let src_end = unsafe { cur_src.add(src_size as usize) };
    let mut cur_dest = dest;
    let dest_end = unsafe { cur_dest.add(out_buffer_size as usize) };
    let src_end_fast = unsafe { src_end.sub(RAPIDLZ_COPY_PROTECT_SIZE) };
    let dest_end_fast = unsafe { dest_end.sub(RAPIDLZ_COPY_PROTECT_SIZE) };
    let dict_end = unsafe { dict_start.add(dict_size as usize) };

    let mut token: u32;
    let mut len: u32;
    let mut offset: u16;
    let mut match_src: *const u8;
    let mut temp: u32 = 0;
    let mut left_src_size: usize;

    loop {
        token = unsafe { *cur_src } as u32;
        cur_src = unsafe { cur_src.add(1) };

        len = token >> 4;
        if rapidlz_likely(len < RAPIDLZ_MAX_4BIT_VALUE) {
            if rapidlz_likely(
                (unsafe { cur_src.add(len as usize) } <= src_end_fast)
                    && (unsafe { cur_dest.add(len as usize) } <= dest_end_fast),
            ) {
                rapidlz_copy_16_byte(cur_dest, cur_src);
                cur_src = unsafe { cur_src.add(len as usize) };
                cur_dest = unsafe { cur_dest.add(len as usize) };
            } else {
                left_src_size = unsafe { src_end.offset_from(cur_src) } as usize;
                let copy_len = len as usize;
                if left_src_size < copy_len {
                    unsafe {
                        ptr::copy_nonoverlapping(cur_src, cur_dest, left_src_size);
                        cur_src = cur_src.add(left_src_size);
                        cur_dest = cur_dest.add(left_src_size);
                    }
                } else {
                    unsafe {
                        ptr::copy_nonoverlapping(cur_src, cur_dest, copy_len);
                        cur_src = cur_src.add(copy_len);
                        cur_dest = cur_dest.add(copy_len);
                    }
                }
            }
        } else {
            // RAPIDLZ_READ_OPTIONAL_LENGTH implementation
            if unsafe { cur_src >= src_end } {
                break;
            }
            len += unsafe { *cur_src } as u32;
            cur_src = unsafe { cur_src.add(1) };
            if len == (RAPIDLZ_MAX_4BIT_VALUE + 0xFF) {
                if unsafe { cur_src >= src_end } {
                    break;
                }
                len += unsafe { *cur_src } as u32;
                cur_src = unsafe { cur_src.add(1) };
                temp = len << 8;
                if unsafe { cur_src >= src_end } {
                    break;
                }
                len = temp + unsafe { *cur_src } as u32;
                cur_src = unsafe { cur_src.add(1) };
            }

            if rapidlz_likely(
                (unsafe { cur_src.add(len as usize) } <= src_end_fast)
                    && (unsafe { cur_dest.add(len as usize) } <= dest_end_fast),
            ) {
                rapidlz_wild_copy_16(cur_src, cur_dest, unsafe { cur_dest.add(len as usize) });
                cur_src = unsafe { cur_src.add(len as usize) };
                cur_dest = unsafe { cur_dest.add(len as usize) };
            } else {
                left_src_size = unsafe { src_end.offset_from(cur_src) } as usize;
                let copy_len = len as usize;
                if left_src_size < copy_len {
                    unsafe {
                        ptr::copy_nonoverlapping(cur_src, cur_dest, left_src_size);
                        cur_src = cur_src.add(left_src_size);
                        cur_dest = cur_dest.add(left_src_size);
                    }
                } else {
                    unsafe {
                        ptr::copy_nonoverlapping(cur_src, cur_dest, copy_len);
                        cur_src = cur_src.add(copy_len);
                        cur_dest = cur_dest.add(copy_len);
                    }
                }
            }
        }

        if unsafe { cur_src.add(2) > src_end } {
            break;
        }
        offset = rapidlz_read_le_16_bit(cur_src);
        cur_src = unsafe { cur_src.add(2) };
        match_src = unsafe { cur_dest.offset(-(offset as isize)) };

        len = token & RAPIDLZ_MAX_4BIT_VALUE;

        // RAPIDLZ_GET_MATCH_LEN implementation
        if len == RAPIDLZ_MAX_4BIT_VALUE {
            if unsafe { cur_src >= src_end } {
                break;
            }
            len += unsafe { *cur_src } as u32;
            cur_src = unsafe { cur_src.add(1) };
            if len == (RAPIDLZ_MAX_4BIT_VALUE + 0xFF) {
                if unsafe { cur_src >= src_end } {
                    break;
                }
                len += unsafe { *cur_src } as u32;
                cur_src = unsafe { cur_src.add(1) };
                temp = len << 8;
                if unsafe { cur_src >= src_end } {
                    break;
                }
                len = temp + unsafe { *cur_src } as u32;
                cur_src = unsafe { cur_src.add(1) };
            }
        }

        if unsafe { cur_dest.add(len as usize) > dest_end.sub(RAPIDLZ_LAST_LITERALS) } {
            break;
        }

        if match_src >= dest {
            if rapidlz_likely(
                unsafe { cur_dest.add(len as usize) }
                    <= dest_end_fast.sub(RAPIDLZ_COPY_PROTECT_SIZE - RAPIDLZ_LAST_LITERALS),
            ) {
                rapidlz_copy_match_fast(cur_dest, match_src, offset, len);
                cur_dest = unsafe { cur_dest.add(len as usize) };
            } else {
                if rapidlz_likely(len < 1024) {
                    rapidlz_fast_safe_copy_by_bytes(cur_dest, match_src, len as usize);
                    cur_dest = unsafe { cur_dest.add(len as usize) };
                } else {
                    rapidlz_safe_copy_match_fast(cur_dest, match_src, dest_end, offset, len);
                    cur_dest = unsafe { cur_dest.add(len as usize) };
                }
            }
        } else {
            let dest_ptr = dest as *const u8;
            if len as usize <= unsafe { dest.offset_from(match_src) } as usize {
                let copy_src = unsafe { dict_end.offset(-(dest.offset_from(match_src) as isize)) };
                unsafe {
                    ptr::copy_nonoverlapping(copy_src, cur_dest, len as usize);
                }
                cur_dest = unsafe { cur_dest.add(len as usize) };
            } else {
                let extern_copy_size = unsafe { dest.offset_from(match_src) } as usize;
                let inner_copy_size = len as usize - extern_copy_size;
                let copy_src = unsafe { dict_end.offset(-(extern_copy_size as isize)) };
                unsafe {
                    ptr::copy_nonoverlapping(copy_src, cur_dest, extern_copy_size);
                }
                cur_dest = unsafe { cur_dest.add(extern_copy_size) };
                if inner_copy_size > unsafe { cur_dest.offset_from(dest) } as usize {
                    let mut copy_src = dest;
                    for _ in 0..inner_copy_size {
                        unsafe {
                            *cur_dest = *copy_src;
                            cur_dest = cur_dest.add(1);
                            copy_src = copy_src.add(1);
                        }
                    }
                } else {
                    unsafe {
                        ptr::copy_nonoverlapping(dest, cur_dest, inner_copy_size);
                    }
                    cur_dest = unsafe { cur_dest.add(inner_copy_size) };
                }
            }
        }
    }

    unsafe { cur_dest.offset_from(dest) } as i32
}
