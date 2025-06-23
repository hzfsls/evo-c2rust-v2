macro_rules! CMPTLZ_ALIGN { () => { CMPTLZ_IS_MATCH!() + (CMPTLZ_PB_STATE_NUM_ALIGN!() << CMPTLZ_PB_BITS_MAX!()) } }
pub(crate) use CMPTLZ_ALIGN;
