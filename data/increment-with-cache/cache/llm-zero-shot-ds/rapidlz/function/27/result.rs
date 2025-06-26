#[inline]
fn rapidlz_copy_16_byte(dst: &mut [u8], src: &[u8]) {
    assert!(dst.len() >= 16 && src.len() >= 16, "Source and destination must have at least 16 bytes");
    
    #[cfg(target_feature = "neon")]
    unsafe {
        use std::arch::aarch64::*;
        let data = vld1q_u8(src.as_ptr());
        vst1q_u8(dst.as_mut_ptr(), data);
    }
    
    #[cfg(target_feature = "sse2")]
    unsafe {
        use std::arch::x86_64::*;
        let data = _mm_loadu_si128(src.as_ptr() as *const __m128i);
        _mm_storeu_si128(dst.as_mut_ptr() as *mut __m128i, data);
    }
    
    #[cfg(not(any(target_feature = "neon", target_feature = "sse2")))]
    {
        use std::ptr::copy_nonoverlapping;
        unsafe {
            copy_nonoverlapping(src.as_ptr(), dst.as_mut_ptr(), 16);
        }
    }
}
