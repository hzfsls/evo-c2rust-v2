pub fn RapidlzCopy16Byte(mut dst: Ptr<Void>, mut src: Ptr<Void>) {
    RAPIDLZ_WRITE64BIT!(dst, RAPIDLZ_READ64BIT!(src));
    RAPIDLZ_WRITE64BIT!(dst.cast::<Ptr<u8>>() + 8, RAPIDLZ_READ64BIT!(src.cast::<Ptr<u8>>() + 8));
}
