pub fn CmptlzSwap32(mut val: u32) -> u32 {
    #[cfg(all(target_arch = "x86_64", target_feature = "bswap"))]
    {
        return val.swap_bytes();
    }
    #[cfg(not(all(target_arch = "x86_64", target_feature = "bswap")))]
    {
        return ((0xff000000 & (val << 24)) | ((0x000000ff & (val >> 24))) | ((0x00ff0000 & (val << 8))) |
                ((0x0000ff00 & (val >> 8))));
    }
}
