const RAPIDLZ_SIXTEEN_BYTE: u16 = 16;
const RAPIDLZ_EIGHT_BYTE: usize = 8;

// Assuming g_overlapOffAddVal is defined somewhere as:
// static g_overlapOffAddVal: [usize; RAPIDLZ_EIGHT_BYTE] = [...];

unsafe fn RapidlzCopyMatchFast(dst: *mut u8, match_: *const u8, offset: u16, length: u32) {
    let mut dst_curr = dst;
    let mut match_ptr = match_;

    if offset >= RAPIDLZ_SIXTEEN_BYTE {
        RapidlzCopyLiteralsFast(match_ptr, dst_curr, length);
        return;
    }

    for i in 0..RAPIDLZ_EIGHT_BYTE {
        *dst_curr.add(i) = *match_ptr.add(i);
    }

    if length <= RAPIDLZ_EIGHT_BYTE as u32 {
        return;
    }

    let dst_end = dst_curr.add(length as usize);
    if offset < RAPIDLZ_EIGHT_BYTE as u16 {
        match_ptr = match_ptr.add(g_overlapOffAddVal[offset as usize]);
        dst_curr = dst_curr.add(RAPIDLZ_EIGHT_BYTE);
    }

    while dst_curr < dst_end {
        RapidlzCopy8Byte(dst_curr, match_ptr);
        dst_curr = dst_curr.add(RAPIDLZ_EIGHT_BYTE);
        match_ptr = match_ptr.add(RAPIDLZ_EIGHT_BYTE);
    }
}
