pub fn PosSlotHelper(mut n: u32) -> u32 {
    #[cfg(all(defined(__GNUC__), __GNUC__ >= 3))]
    return (31 - __builtin_clz!(n).cast::<u32>());
    #[cfg(not(all(defined(__GNUC__), __GNUC__ >= 3)))]
    let mut i: u32 = 31;
    if (n & 0xFFFF0000) == 0 {
        n <<= 16;
        i = 15;
    }
    if (n & 0xFF000000) == 0 {
        n <<= 8;
        i -= 8;
    }
    if (n & 0xF0000000) == 0 {
        n <<= 4;
        i -= 4;
    }
    if (n & 0xC0000000) == 0 {
        n <<= 2;
        i -= 2;
    }
    if (n & 0x80000000) == 0 {
        i -= 1;
    }
    return i;
}
