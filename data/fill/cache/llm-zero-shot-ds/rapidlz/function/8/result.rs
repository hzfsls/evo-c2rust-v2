unsafe fn rapidlz_wild_copy_16(src_ptr: *const u8, dst_ptr: *mut u8, dst_end: *mut u8) {
    let mut tmp_dst_ptr = dst_ptr;
    let mut tmp_src_ptr = src_ptr;
    while tmp_dst_ptr < dst_end {
        rapidlz_copy_16_byte(tmp_dst_ptr, tmp_src_ptr);
        tmp_dst_ptr = tmp_dst_ptr.wrapping_add(16);
        tmp_src_ptr = tmp_src_ptr.wrapping_add(16);
    }
}
