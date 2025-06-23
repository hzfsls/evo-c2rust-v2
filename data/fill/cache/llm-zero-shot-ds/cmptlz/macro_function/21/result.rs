macro_rules! cmptlz_set_dictsize_by_level {
    ($level:expr, $dictSize:expr) => {
        $dictSize = if $level <= 5 {
            1 << ($level * 2 + 14)
        } else if $level <= 7 {
            1 << 25
        } else {
            1 << 26
        };
    };
}

pub(crate) use cmptlz_set_dictsize_by_level;
