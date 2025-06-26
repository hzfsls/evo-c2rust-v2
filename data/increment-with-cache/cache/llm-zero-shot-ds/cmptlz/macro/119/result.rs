macro_rules! CMPTLZ_ALIGN {
    () => {
        $crate::CMPTLZ_IS_MATCH + ($crate::CMPTLZ_PB_STATE_NUM_ALIGN << $crate::CMPTLZ_PB_BITS_MAX)
    };
}
pub(crate) use CMPTLZ_ALIGN;
