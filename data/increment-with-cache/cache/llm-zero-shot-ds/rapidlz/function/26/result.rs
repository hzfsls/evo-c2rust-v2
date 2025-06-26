#[inline]
pub unsafe fn rapidlz_copy_32_byte(dst: *mut u8, src: *const u8) {
    #[cfg(target_feature = "neon")]
    {
        use std::arch::aarch64::*;
        vst1q_u8(dst, vld1q_u8(src));
        vst1q_u8(dst.add(16), vld1q_u8(src.add(16)));
    }
    
    #[cfg(target_feature = "sse2")]
    {
        use std::arch::x86_64::*;
        _mm_storeu_si128(dst as *mut __m128i, _mm_loadu_si128(src as *const __m128i));
        _mm_storeu_si128(dst.add(16) as *mut __m128i, _mm_loadu_si128(src.add(16) as *const __m128i));
    }
    
    #[cfg(not(any(target_feature = "neon", target_feature = "sse2")))]
    {
        use std::ptr::{read_unaligned, write_unaligned};
        write_unaligned(dst as *mut u64, read_unaligned(src as *const u64));
        write_unaligned(dst.add(8) as *mut u64, read_unaligned(src.add(8) as *const u64));
        write_unaligned(dst.add(16) as *mut u64, read_unaligned(src.add(16) as *const u64));
        write_unaligned(dst.add(24) as *mut u64, read_unaligned(src.add(24) as *const u64));
    }
}
