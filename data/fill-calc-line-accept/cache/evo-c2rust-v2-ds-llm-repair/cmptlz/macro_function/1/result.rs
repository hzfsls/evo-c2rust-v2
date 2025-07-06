macro_rules! CMPTLZ_ERROR_CONVERT { ($x:expr) => { ((CMPTLZ_MODULE!() << 16) | ($x as u32)) as i32 } }
pub(crate) use CMPTLZ_ERROR_CONVERT;
