#[inline]
fn rapidlz_wild_copy_32(src_ptr: &[u8], dst_ptr: &mut [u8], dst_end: *const u8) {
    let mut tmp_dst_ptr = dst_ptr.as_mut_ptr();
    let mut tmp_src_ptr = src_ptr.as_ptr();
    while tmp_dst_ptr < dst_end {
        // Assuming RapidlzCopy32Byte is a function that copies 32 bytes from src to dst
        // For the purpose of this translation, we'll use copy_from_slice
        // Note: This assumes dst_ptr has at least 32 bytes remaining
        unsafe {
            std::ptr::copy_nonoverlapping(tmp_src_ptr, tmp_dst_ptr, 32);
        }
        tmp_dst_ptr = unsafe { tmp_dst_ptr.add(32) };
        tmp_src_ptr = unsafe { tmp_src_ptr.add(32) };
    }
}
