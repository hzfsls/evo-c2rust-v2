pub fn RapidlzCountTailZero64(mut x: u64) -> u8 {
    #[cfg(all(defined(__GNUC__), __GNUC__ >= 3))]
    return __builtin_ctzll!(x).cast::<u8>();
    if (x == 0).as_bool() {
        return 0;
    }
    let mut val: u64 = x.cast();
    let mut num: u8 = 0;
    while ((val & 1) == 0).as_bool() {
        val >>= 1;
        num += 1;
    }
    return num.cast();
}
