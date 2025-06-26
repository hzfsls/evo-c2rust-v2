macro_rules! CMPTLZ_SET_DICTSIZE_BY_LEVEL {
    ($level:expr, $dictSize:expr) => {
        if $level <= 5 {
            $dictSize = 1 << ($level * 2 + 14);
        } else if $level <= 7 {
            $dictSize = 1 << 25;
        } else {
            $dictSize = 1 << 26;
        }
    };
}
pub(crate) use CMPTLZ_SET_DICTSIZE_BY_LEVEL;