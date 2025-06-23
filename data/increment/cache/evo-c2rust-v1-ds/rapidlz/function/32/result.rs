pub fn RapidlzCopyLiteralsFast(mut src: Ptr<u8>, mut dst: Ptr<u8>, mut length: u32) {
    if RAPIDLZ_LIKELY!(length <= RAPIDLZ_SIXTEEN_BYTE!()) {
        RapidlzCopy16Byte(dst.cast(), src.cast());
        return;
    }
    RapidlzWildCopy16(src.cast(), dst.cast(), (dst + length).cast());
}
