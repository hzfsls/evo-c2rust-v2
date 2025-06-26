macro_rules! CMPTLZ_IS_MATCH { () => { CMPTLZ_MATCH_LEN_CODER!() + CMPTLZ_LENPROB_NUM!() } }
pub(crate) use CMPTLZ_IS_MATCH;
