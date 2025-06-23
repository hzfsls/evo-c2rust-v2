fn rapidlz_high_bit64(x: u64) -> u8 {
    assert!(x != 0, "x must not be zero");
    let mut pos = 64;
    let mut value = x;
    if value == 0 {
        return 0;
    }
    if (value & 0xFFFFFFFF00000000) == 0 {
        value <<= 32;
        pos -= 32;
    }
    if (value & 0xFFFF000000000000) == 0 {
        value <<= 16;
        pos -= 16;
    }
    if (value & 0xFF00000000000000) == 0 {
        value <<= 8;
        pos -= 8;
    }
    if (value & 0xF000000000000000) == 0 {
        value <<= 4;
        pos -= 4;
    }
    if (value & 0xC000000000000000) == 0 {
        value <<= 2;
        pos -= 2;
    }
    if (value & 0x8000000000000000) == 0 {
        value <<= 1;
        pos -= 1;
    }
    pos - 1
}
