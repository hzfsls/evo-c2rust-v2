pub fn RapidlzDecompressSafeUsingDict(mut src: Ptr<u8>, mut dst: Ptr<u8>, mut compressedSize: i32, mut dstSize: i32, mut dictStart: Ptr<u8>, mut dictSize: i32) -> i32 {
    if src == NULL!() || compressedSize == 0 || dst == NULL!() || dstSize < 0 {
        return RAPIDLZ_ERROR_PARAM_UNSUPPORTED!();
    }
    if RAPIDLZ_UNLIKELY!(dstSize == 0) {
        return RapidlzZeroBytesDecode(src.cast(), compressedSize.cast());
    }
    let mut rapidlzDecFunc: RapidlzDecompressFunc;
    if dictSize == 0 || dictStart + dictSize == dst {
        rapidlzDecFunc = RapidlzDecWithPrefixDict;
    } else {
        rapidlzDecFunc = RapidlzDecWithExternalDict;
    }
    return rapidlzDecFunc(src.cast(), dst.cast(), compressedSize.cast(), dstSize.cast(), dictStart.cast(), dictSize.cast());
}
