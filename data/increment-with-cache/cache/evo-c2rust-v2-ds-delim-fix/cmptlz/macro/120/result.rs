macro_rules! CMPTLZ_ISREP { () => { CMPTLZ_ALIGN!() + CMPTLZ_ALIGN_TABLE_SIZE!() } }
pub(crate) use CMPTLZ_ISREP;
