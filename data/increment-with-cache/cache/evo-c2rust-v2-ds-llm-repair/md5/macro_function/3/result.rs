macro_rules! MD5_LINEAR_FUNC_I { ($B:expr, $C:expr, $D:expr) => { $C ^ ($B | !$D) } }
pub(crate) use MD5_LINEAR_FUNC_I;
