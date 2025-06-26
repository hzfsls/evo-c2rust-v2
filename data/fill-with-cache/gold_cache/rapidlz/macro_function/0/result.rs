macro_rules! RAPIDLZ_LOG {
    ($error_code:expr, $fmt:expr) => {
        RapidlzLogWrite($error_code as usize, RAPIDLZFILENAME!(), c__line__!().cast(), $fmt, &[]);
    };
    ($error_code:expr, $fmt:expr, $($args:expr), *) => {
        RapidlzLogWrite($error_code as usize, RAPIDLZFILENAME!(), c__line__!().cast(), $fmt, &[$(&$args), *]);
    };
}
pub(crate) use RAPIDLZ_LOG;