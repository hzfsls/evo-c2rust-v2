pub fn RapidlzZeroBytesDecode(mut src: Ptr<u8>, mut srcSize: i32) -> i32 {
    if *src == 0 && srcSize == 1 {
        return RAPIDLZ_DEC_NOT_OK!();
    }
    return RAPIDLZ_ERROR_PARAM_UNSUPPORTED!();
}
