fn rapidlz_is_le() -> bool {
    let n = 1i32;
    unsafe { *(n as *const i32 as *const u8) == 1u8 }
}
