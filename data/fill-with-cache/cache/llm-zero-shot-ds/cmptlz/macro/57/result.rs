macro_rules! CMPTLZ_MATCH_LEN_CODER {
    () => {
        CMPTLZ_REP_LEN_CODER!() + CMPTLZ_LENPROB_NUM!()
    };
}
pub(crate) use CMPTLZ_MATCH_LEN_CODER;
