macro_rules! RAPIDLZ_RETURN_IF_NOT_EOK {
    ($condition:expr, $errCode:expr) => {
        if $condition != EOK {
            RAPIDLZ_LOG!($errCode, " ");
            return $errCode;
        }
    };
}

pub(crate) use RAPIDLZ_RETURN_IF_NOT_EOK;
