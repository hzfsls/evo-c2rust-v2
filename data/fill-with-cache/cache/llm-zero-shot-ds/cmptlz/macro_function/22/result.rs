macro_rules! cmptlz_set_fb_by_level {
    ($level:expr, $fastBytes:expr) => {
        $fastBytes = if $level < 7 { 32 } else { 64 };
    };
}

pub(crate) use cmptlz_set_fb_by_level;
