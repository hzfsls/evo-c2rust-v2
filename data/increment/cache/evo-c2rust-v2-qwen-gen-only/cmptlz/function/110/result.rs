pub fn PosSlotHelper(mut n: u32) -> u32 {
    #[cfg(all(target_env = "gnu", target_arch = "x86_64"))]
    {
        return 31 - (n.leading_zeros() as u32);
    }
    #[cfg(not(all(target_env = "gnu", target_arch = "x86_64")))]
    {
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
}