pub fn RapidlzCopy8Byte(mut dst: Ptr<Void>, mut src: Ptr<Void>) {
    #[cfg(ARM_NEON)]
    {
        vst1_u8!(dst.cast::<Ptr<u8>>(), vld1_u8!(src.cast::<Ptr<u8>>()));
    }
    #[cfg(not(ARM_NEON))]
    {
        RAPIDLZ_WRITE64BIT!(dst, RAPIDLZ_READ64BIT!(src));
    }
}
