macro_rules! CMPT_GET_DIST_STATE { ($len:expr) => { if $len < 4 + CMPTLZ_MATCH_LEN_MIN!() { $len - CMPTLZ_MATCH_LEN_MIN!() } else { 4 - 1 } } }
pub(crate) use CMPT_GET_DIST_STATE;
