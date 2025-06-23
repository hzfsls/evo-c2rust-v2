pub fn RapidlzCopy16Byte(mut dst: Ptr<Void>, mut src: Ptr<Void>) {
    #[cfg(ARM_NEON)]
    {
        vst1q_u8!(dst.cast::<Ptr<u8>>(), vld1q_u8!(src.cast::<Ptr<u8>>()));
    }
    #[cfg(X86_SSE2)]
    {
        _mm_storeu_si128!(dst.cast::<Ptr<__m128i>>(), _mm_loadu_si128!(src.cast::<Ptr<__m128i>>()));
    }
    #[cfg(not(any(ARM_NEON, X86_SSE2)))]
    {
        RAPIDLZ_WRITE64BIT!(dst.cast(), RAPIDLZ_READ64BIT!(src.cast()));
        RAPIDLZ_WRITE64BIT!((dst.cast::<Ptr<u8>>() + 8).cast(), RAPIDLZ_READ64BIT!((src.cast::<Ptr<u8>>() + 8).cast()));
    }
}
