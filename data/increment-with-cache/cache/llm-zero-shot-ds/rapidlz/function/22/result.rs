#[inline]
fn rapidlz_count_tail_zero64(x: u64) -> u8 {
    if x == 0 {
        return 0;
    }
    let mut val = x;
    let mut num = 0;
    while (val & 1) == 0 {
        val >>= 1;
        num += 1;
    }
    num
}
