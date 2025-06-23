pub fn CmptlzSwap32(mut val: u32) -> u32 {
    #[cfg(all(__GNUC__, __GNUC__ > 4 || (__GNUC__ == 4 && __GNUC_MINOR__ >= 2)))]
    {
        return __builtin_bswap32!(val).cast::<u32>();
    }
    #[cfg(not(all(__GNUC__, __GNUC__ > 4 || (__GNUC__ == 4 && __GNUC_MINOR__ >= 2))))]
    {
        return ((0xff000000 & (val << 24)) | (0x000000ff & (val >> 24)) | (0x00ff0000 & (val << 8)) | (0x0000ff00 & (val >> 8))).cast();
    }
}
