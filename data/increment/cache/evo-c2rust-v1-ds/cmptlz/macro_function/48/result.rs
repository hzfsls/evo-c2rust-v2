macro_rules! CMPTLZ_LOG {
    ($error_code:expr, $fmt:expr) => {
        CmptlzLogWrite($error_code as usize, __FUNCTION__!().cast(), __LINE__!().cast(), $fmt.cast(), &[]);
    };
    ($error_code:expr, $fmt:expr, $($args:expr),*) => {
        CmptlzLogWrite($error_code as usize, __FUNCTION__!().cast(), __LINE__!().cast(), $fmt.cast(), &[$(&$args), *]);
    };
}
pub(crate) use CMPTLZ_LOG;
