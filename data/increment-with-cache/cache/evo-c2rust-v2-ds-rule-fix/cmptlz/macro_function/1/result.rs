macro_rules! CMPTLZ_HANDLE_CONVERT { ($x:expr) => { (CMPTLZ_MODULE!() << 16 | ($x as u32) << 8) as i32 } }
pub(crate) use CMPTLZ_HANDLE_CONVERT;
