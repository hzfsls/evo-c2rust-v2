#[inline]
fn rapidlz_wild_copy8(src_ptr: &[u8], dst_ptr: &mut [u8], dst_end: usize) {
    let mut tmp_dst_ptr = dst_ptr.as_mut_ptr();
    let mut tmp_src_ptr = src_ptr.as_ptr();
    let dst_end_ptr = dst_ptr.as_mut_ptr().wrapping_add(dst_end);
    
    while tmp_dst_ptr < dst_end_ptr {
        unsafe {
            std::ptr::copy_nonoverlapping(tmp_src_ptr, tmp_dst_ptr, 8);
            tmp_dst_ptr = tmp_dst_ptr.wrapping_add(8);
            tmp_src_ptr = tmp_src_ptr.wrapping_add(8);
        }
    }
}
