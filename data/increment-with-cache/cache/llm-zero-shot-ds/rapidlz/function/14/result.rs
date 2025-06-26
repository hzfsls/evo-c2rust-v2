use std::ptr;

static g_enc32table: [usize; RAPIDLZ_EIGHT_BYTE] = [/* ... */];
static g_dec64table: [usize; RAPIDLZ_EIGHT_BYTE] = [/* ... */];

const RAPIDLZ_FOUR_BYTE: usize = 4;
const RAPIDLZ_EIGHT_BYTE: usize = 8;

#[inline]
fn rapidlz_safe_copy_match_fast(
    cur_dest: *mut u8,
    match_src: *mut u8,
    dest_end: *mut u8,
    offset: u16,
    len: u32,
) {
    let cur_dest_copy_end = unsafe { cur_dest.add(len as usize) };
    
    if offset < RAPIDLZ_EIGHT_BYTE as u16 {
        unsafe {
            *cur_dest = *match_src;
            *cur_dest.add(1) = *match_src.add(1);
            *cur_dest.add(2) = *match_src.add(2);
            *cur_dest.add(3) = *match_src.add(3);
            
            let match_src = match_src.add(g_enc32table[offset as usize]);
            ptr::copy_nonoverlapping(
                match_src,
                cur_dest.add(RAPIDLZ_FOUR_BYTE),
                RAPIDLZ_FOUR_BYTE,
            );
            let match_src = match_src.sub(g_dec64table[offset as usize]);
        }
    } else {
        unsafe {
            ptr::copy_nonoverlapping(
                match_src,
                cur_dest,
                RAPIDLZ_EIGHT_BYTE,
            );
            let match_src = match_src.add(RAPIDLZ_EIGHT_BYTE);
        }
    }
    
    let cur_dest = unsafe { cur_dest.add(RAPIDLZ_EIGHT_BYTE) };
    let cur_dest_limit = unsafe { dest_end.sub(RAPIDLZ_EIGHT_BYTE - 1) };
    
    if cur_dest < cur_dest_limit {
        rapidlz_wild_copy_8(match_src, cur_dest, cur_dest_limit);
        let bytes_copied = unsafe { cur_dest_limit.offset_from(cur_dest) } as usize;
        let match_src = unsafe { match_src.add(bytes_copied) };
        cur_dest = cur_dest_limit;
    }
    
    while cur_dest < cur_dest_copy_end {
        unsafe {
            *cur_dest = *match_src;
            cur_dest = cur_dest.add(1);
            match_src = match_src.add(1);
        }
    }
}
