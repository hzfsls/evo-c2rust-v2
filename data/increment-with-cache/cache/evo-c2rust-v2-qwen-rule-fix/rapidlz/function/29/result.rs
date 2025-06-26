pub fn RapidlzCopy16Byte(mut dst: Ptr<Void>, mut src: Ptr<Void>) {
    if ARM_NEON!() {
        vst1q_u8!(dst.cast::<Ptr<u8>>(), vld1q_u8!(src.cast::<Ptr<u8>>()));
    } else if X86_SSE2!() {
        _mm_storeu_si128!(dst.cast::<Ptr<__m128i>>(), _mm_loadu_si128!(src.cast::<Ptr<__m128i>>()));
    } else {
        RAPIDLZ_WRITE64BIT!(dst, RAPIDLZ_READ64BIT!(src));
        RAPIDLZ_WRITE64BIT!((dst.cast::<Ptr<u8>>() + 8), RAPIDLZ_READ64BIT!((src.cast::<Ptr<u8>>() + 8)));
    }
}