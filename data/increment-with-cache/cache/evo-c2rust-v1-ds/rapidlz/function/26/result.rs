pub fn RapidlzCopy32Byte(mut dst: Ptr<Void>, mut src: Ptr<Void>) {
    #[cfg(ARM_NEON)]
    {
        vst1q_u8!(dst.cast::<Ptr<u8>>(), vld1q_u8!(src.cast::<Ptr<u8>>()));
        vst1q_u8!(dst.cast::<Ptr<u8>>() + 16, vld1q_u8!(src.cast::<Ptr<u8>>() + 16));
    }
    #[cfg(X86_SSE2)]
    {
        _mm_storeu_si128!(dst.cast::<Ptr<__m128i>>(), _mm_loadu_si128!(src.cast::<Ptr<__m128i>>()));
        _mm_storeu_si128!(dst.cast::<Ptr<__m128i>>() + 1, _mm_loadu_si128!(src.cast::<Ptr<__m128i>>() + 1));
    }
    #[cfg(not(any(ARM_NEON, X86_SSE2)))]
    {
        RAPIDLZ_WRITE64BIT!(dst, RAPIDLZ_READ64BIT!(src));
        RAPIDLZ_WRITE64BIT!(dst.cast::<Ptr<u8>>() + 8, RAPIDLZ_READ64BIT!(src.cast::<Ptr<u8>>() + 8));
        RAPIDLZ_WRITE64BIT!(dst.cast::<Ptr<u8>>() + 16, RAPIDLZ_READ64BIT!(src.cast::<Ptr<u8>>() + 16));
        RAPIDLZ_WRITE64BIT!(dst.cast::<Ptr<u8>>() + 24, RAPIDLZ_READ64BIT!(src.cast::<Ptr<u8>>() + 24));
    }
}
