macro_rules! RAPIDLZ_LOG {
    ($error_code:expr, $fmt:expr) => {
        RapidlzLogWrite($error_code as usize, RAPIDLZFILENAME!().cast(), __LINE__!().cast(), $fmt.cast(), &[]);
    };
    ($error_code:expr, $fmt:expr, $($args:expr),*) => {
        RapidlzLogWrite($error_code as usize, RAPIDLZFILENAME!().cast(), __LINE__!().cast(), $fmt.cast(), &[$(&$args), *]);
    };
}
pub(crate) use RAPIDLZ_LOG;
