#[inline]
fn rapidlz_copy_8_byte(dst: &mut [u8], src: &[u8]) {
    #[cfg(target_feature = "neon")]
    unsafe {
        use std::arch::aarch64::*;
        vst1_u8(dst.as_mut_ptr(), vld1_u8(src.as_ptr()));
    }
    
    #[cfg(not(target_feature = "neon"))]
    {
        assert!(dst.len() >= 8 && src.len() >= 8);
        let value = u64::from_le_bytes(src[..8].try_into().unwrap());
        dst[..8].copy_from_slice(&value.to_le_bytes());
    }
}
