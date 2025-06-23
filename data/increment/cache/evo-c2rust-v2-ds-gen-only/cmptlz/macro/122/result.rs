macro_rules! CMPTLZ_ISREPG1 { () => { CMPTLZ_ISREPG0!() + CMPTLZ_MKSTATE_NUM!() } }
pub(crate) use CMPTLZ_ISREPG1;
