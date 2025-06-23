pub fn RapidlzIsLE() -> i32 {
    #[cfg(any(target_env = "gnu", target_env = "clang"))]
    {
        return if __BYTE_ORDER__ == __ORDER_LITTLE_ENDIAN__ { 1 } else { 0 };
    }
    let mut n: i32 = 1;
    return (*c_ref!(n).cast::<Ptr<u8>>())[0].cast();
}