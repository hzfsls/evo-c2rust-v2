macro_rules! CMPTLZ_REP_LEN_CODER {
    () => {
        CMPTLZ_REP0_LONG + (CMPTLZ_PB_STATE_NUM_ALIGN << CMPTLZ_PB_BITS_MAX)
    };
}
pub(crate) use CMPTLZ_REP_LEN_CODER;
