macro_rules! RAPIDLZ_RETURN_IF_NOT_TRUE {
    ($condition:expr, $errCode:expr) => {
        if !$condition {
            RAPIDLZ_LOG!($errCode, cstr!(" "));
            return $errCode;
        }
    }
}
pub(crate) use RAPIDLZ_RETURN_IF_NOT_TRUE;
