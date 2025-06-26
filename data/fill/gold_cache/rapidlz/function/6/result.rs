pub fn RapidlzCopy16Byte(mut dst: VoidPtr, mut src: VoidPtr) {
    RAPIDLZ_WRITE64BIT!(dst, RAPIDLZ_READ64BIT!(src));
    RAPIDLZ_WRITE64BIT!(
        dst.cast::<Ptr<u8>>() + 8,
        RAPIDLZ_READ64BIT!(src.cast::<Ptr<u8>>() + 8)
    );
}