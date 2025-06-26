pub fn rapidlz_count_lead_zero64(x: u64) -> u8 {
    if x == 0 {
        return 0;
    }
    let mut num = 0;
    let mut val = x;
    while (val & 0x8000000000000000) == 0 {
        val <<= 1;
        num += 1;
    }
    num
}
