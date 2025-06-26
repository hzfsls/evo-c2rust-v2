macro_rules! CMPTLZ_SET_FB_BY_LEVEL { ($level:expr, $fastBytes:expr) => { $fastBytes = if $level < 7 { 32 } else { 64 } } }
pub(crate) use CMPTLZ_SET_FB_BY_LEVEL;
