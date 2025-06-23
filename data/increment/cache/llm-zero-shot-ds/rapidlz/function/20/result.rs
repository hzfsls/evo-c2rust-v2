#[inline]
fn rapidlz_is_le() -> bool {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        cfg!(target_endian = "little")
    }
    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
    {
        let n = 1;
        unsafe { *(n as *const i32 as *const u8) == 1 }
    }
}
