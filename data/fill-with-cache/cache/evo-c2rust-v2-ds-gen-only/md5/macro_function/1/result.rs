macro_rules! MD5_LINEAR_FUNC_G { ($B:expr, $C:expr, $D:expr) => { ($B & $D) | ($C & !$D) } }
pub(crate) use MD5_LINEAR_FUNC_G;
