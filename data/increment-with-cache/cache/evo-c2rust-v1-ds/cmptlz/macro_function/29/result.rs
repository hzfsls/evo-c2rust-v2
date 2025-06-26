macro_rules! CMPTLZ_SET_DICTSIZE_BY_LEVEL { ($level:expr, $dictSize:expr) =>
    {
        $dictSize = if $level <= 5 {
            1 << ($level * 2 + 14)
        } else if $level <= 7 {
            1 << 25
        } else {
            1 << 26
        };
    }
}
pub(crate) use CMPTLZ_SET_DICTSIZE_BY_LEVEL;
