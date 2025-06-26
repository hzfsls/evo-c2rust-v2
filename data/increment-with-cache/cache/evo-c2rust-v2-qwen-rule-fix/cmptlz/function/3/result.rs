pub fn CmptlzSwap32(mut val: u32) -> u32 {
    if (cfg!(target_feature = "bswap") && (cfg!(target_env = "gnu") && (cfg!(target_version = "4.2") || (cfg!(target_version = "4") && cfg!(target_patch = "2"))))) {
        return __builtin_bswap32!(val);
    }
    return ((0xff000000 & (val << 24)) | (0x000000ff & (val >> 24)) | (0x00ff0000 & (val << 8)) | (0x0000ff00 & (val >> 8)));
}