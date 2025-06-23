pub fn RapidlzCountLeadZero64(mut x: u64) -> u8 {
    if x == 0 {
        return 0;
    }
    let mut num: u8 = 0;
    let mut val: u64 = x;
    while (val & 0x8000000000000000) == 0 {
        val <<= 1;
        num += 1;
    }
    return num;
}