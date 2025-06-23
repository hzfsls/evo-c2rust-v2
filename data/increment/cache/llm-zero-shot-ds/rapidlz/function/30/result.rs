#[inline]
fn rapidlz_wild_copy_16(src_ptr: &[u8], dst_ptr: &mut [u8], dst_end: *const u8) {
    let mut tmp_dst_ptr = dst_ptr.as_mut_ptr();
    let mut tmp_src_ptr = src_ptr.as_ptr();
    while (tmp_dst_ptr as *const u8) < dst_end {
        unsafe {
            std::ptr::copy_nonoverlapping(tmp_src_ptr, tmp_dst_ptr, 16);
        }
        tmp_dst_ptr = unsafe { tmp_dst_ptr.add(16) };
        tmp_src_ptr = unsafe { tmp_src_ptr.add(16) };
    }
}
