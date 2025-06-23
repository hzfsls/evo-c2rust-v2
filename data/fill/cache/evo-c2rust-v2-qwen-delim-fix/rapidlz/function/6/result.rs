pub fn RapidlzCopy16Byte(mut dst: Ptr<Void>, mut src: Ptr<Void>) {
    RAPIDLZ_WRITE64BIT!(dst.cast(), RAPIDLZ_READ64BIT!(src.cast()));
    RAPIDLZ_WRITE64BIT!((dst.cast::<Ptr<u8>>() + 8).cast(), RAPIDLZ_READ64BIT!((src.cast::<Ptr<u8>>() + 8).cast()));
}