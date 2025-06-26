#[inline]
fn cmptlz_swap32(val: u32) -> u32 {
    if cfg!(target_feature = "bswap") {
        val.swap_bytes()
    } else {
        ((0xff000000 & (val << 24)) | (0x000000ff & (val >> 24)) | (0x00ff0000 & (val << 8)) |
         (0x0000ff00 & (val >> 8)))
    }
}
