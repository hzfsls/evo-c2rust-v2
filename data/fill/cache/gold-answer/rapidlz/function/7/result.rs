pub fn RapidlzCopy8Byte(mut dst: VoidPtr, mut src: VoidPtr) {
    RAPIDLZ_WRITE64BIT!(dst, RAPIDLZ_READ64BIT!(src));
}