macro_rules! CMPTLZ_LOG {
    ($error_code:expr, $fmt:expr) => {
        CmptlzLogWrite($error_code as usize, c__function__!(), c__line__!() as u16, $fmt, &[]);
    };
    ($error_code:expr, $fmt:expr, $($args:expr), *) => {
        CmptlzLogWrite($error_code as usize, c__function__!(), c__line__!() as u16, $fmt, &[$(&$args), *]);
    };
}
pub(crate) use CMPTLZ_LOG;