pub fn RapidlzCopy32Byte(mut dst: Ptr<Void>, mut src: Ptr<Void>) {
    if ARM_NEON!().as_bool() {
        vst1q_u8!(dst.cast::<Ptr<u8>>(), vld1q_u8!(src.cast::<Ptr<u8>>()));
        vst1q_u8!((dst.cast::<Ptr<u8>>() + 16), vld1q_u8!((src.cast::<Ptr<u8>>() + 16)));
    } else if X86_SSE2!().as_bool() {
        _mm_storeu_si128!(dst.cast::<Ptr<__m128i>>(), _mm_loadu_si128!(src.cast::<Ptr<__m128i>>()));
        _mm_storeu_si128!((dst.cast::<Ptr<__m128i>>() + 1), _mm_loadu_si128!((src.cast::<Ptr<__m128i>>() + 1)));
    } else {
        RAPIDLZ_WRITE64BIT!(dst.cast::<Ptr<u64>>(), RAPIDLZ_READ64BIT!(src.cast::<Ptr<u64>>()));
        RAPIDLZ_WRITE64BIT!((dst.cast::<Ptr<u8>>() + 8), RAPIDLZ_READ64BIT!((src.cast::<Ptr<u8>>() + 8)));
        RAPIDLZ_WRITE64BIT!((dst.cast::<Ptr<u8>>() + 16), RAPIDLZ_READ64BIT!((src.cast::<Ptr<u8>>() + 16)));
        RAPIDLZ_WRITE64BIT!((dst.cast::<Ptr<u8>>() + 24), RAPIDLZ_READ64BIT!((src.cast::<Ptr<u8>>() + 24)));
    }
}