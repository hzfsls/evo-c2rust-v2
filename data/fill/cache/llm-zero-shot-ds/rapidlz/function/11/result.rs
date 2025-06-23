const RAPIDLZ_SIXTEEN_BYTE: u16 = 16;
const RAPIDLZ_EIGHT_BYTE: usize = 8;

static g_overlapOffAddVal: [usize; RAPIDLZ_EIGHT_BYTE] = [/* appropriate values here */];

fn rapidlz_copy_match_fast(dst: &mut [u8], match_: &[u8], offset: u16, length: u32) {
    let mut dst_curr = dst.as_mut_ptr();
    let mut match_ptr = match_.as_ptr();
    
    if offset >= RAPIDLZ_SIXTEEN_BYTE {
        rapidlz_copy_literals_fast(match_ptr, dst_curr, length);
        return;
    }
    
    unsafe {
        for i in 0..RAPIDLZ_EIGHT_BYTE {
            *dst_curr.add(i) = *match_ptr.add(i);
        }
    }
    
    if length <= RAPIDLZ_EIGHT_BYTE as u32 {
        return;
    }
    
    let dst_end = unsafe { dst_curr.add(length as usize) };
    
    if offset < RAPIDLZ_EIGHT_BYTE as u16 {
        unsafe {
            match_ptr = match_ptr.add(g_overlapOffAddVal[offset as usize]);
            dst_curr = dst_curr.add(RAPIDLZ_EIGHT_BYTE);
        }
    }
    
    while dst_curr < dst_end {
        unsafe {
            rapidlz_copy_8byte(dst_curr, match_ptr);
            dst_curr = dst_curr.add(RAPIDLZ_EIGHT_BYTE);
            match_ptr = match_ptr.add(RAPIDLZ_EIGHT_BYTE);
        }
    }
}
