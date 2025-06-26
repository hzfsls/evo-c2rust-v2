macro_rules! CMPTLZ_CALC_POS_STATE { ($procPos:expr, $pbMask:expr) => { (($procPos) & ($pbMask)) << 4 } }
pub(crate) use CMPTLZ_CALC_POS_STATE;
