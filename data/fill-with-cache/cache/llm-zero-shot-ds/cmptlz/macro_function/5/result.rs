macro_rules! CMPTLZ_LOG {
    ($error_code:expr, $fmt:expr, $($arg:tt)*) => {
        CmptlzLogWrite($error_code as usize, function_name!(), line!(), $fmt, $($arg)*);
    };
}

pub(crate) use CMPTLZ_LOG;
