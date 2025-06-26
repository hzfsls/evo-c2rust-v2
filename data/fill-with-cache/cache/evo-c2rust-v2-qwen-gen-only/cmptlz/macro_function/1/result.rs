macro_rules! CMPTLZ_ERROR_CONVERT { ($x:expr) => { ($x as u32) | ((CMPTLZ_MODULE as u32) << 16) as i32 } }
pub(crate) use CMPTLZ_ERROR_CONVERT;