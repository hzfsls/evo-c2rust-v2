pub fn RapidlzCopy8Byte(mut dst: Ptr<Void>, mut src: Ptr<Void>) {
    RAPIDLZ_WRITE64BIT!(dst.cast(), RAPIDLZ_READ64BIT!(src.cast()));
}