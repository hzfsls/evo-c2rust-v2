macro_rules! CMPTLZ_HANDLE_CONVERT { ($x:expr) => { ($x as u32 << 8) as i32 } }
pub(crate) use CMPTLZ_HANDLE_CONVERT;