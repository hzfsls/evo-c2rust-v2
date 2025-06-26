macro_rules! MD5_LINEAR_FUNC_F { ($B:expr, $C:expr, $D:expr) => { ($B & $C) | ((!$B) & $D) } }
pub(crate) use MD5_LINEAR_FUNC_F;
