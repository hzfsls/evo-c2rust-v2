macro_rules! CMPTLZ_ISREPG2 { () => { CMPTLZ_ISREPG1!() + CMPTLZ_MKSTATE_NUM!() } }
pub(crate) use CMPTLZ_ISREPG2;
