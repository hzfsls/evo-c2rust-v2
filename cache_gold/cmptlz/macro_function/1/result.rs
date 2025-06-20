macro_rules! CMPTLZ_ERROR_CONVERT { ($x:expr) => { (CMPTLZ_MODULE!() << 16 | $x)  } }
pub(crate) use CMPTLZ_ERROR_CONVERT;