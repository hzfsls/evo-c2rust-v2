pub fn PosSlotHelper(mut n: u32) -> u32 {
    let mut i: u32 = 31;
    if (n & 0xFFFF0000 == 0).as_bool() {
        n <<= 16;
        i = 15;
    }
    if (n & 0xFF000000 == 0).as_bool() {
        n <<= 8;
        i -= 8;
    }
    if (n & 0xF0000000 == 0).as_bool() {
        n <<= 4;
        i -= 4;
    }
    if (n & 0xC0000000 == 0).as_bool() {
        n <<= 2;
        i -= 2;
    }
    if (n & 0x80000000 == 0).as_bool() {
        i -= 1;
    }
    return i.cast();
}
