macro_rules! CMPTLZ_IS_THE_BIT_0 { ($probSlot:expr, $range:expr, $rangeCode:expr, $rangeBound:expr) =>
    {
        {
        $rangeBound = ($range >> CMPTLZ_PROB_LG_BIT!()) * (*$probSlot as u32);
        $rangeCode < $rangeBound
        }
    }
}
pub(crate) use CMPTLZ_IS_THE_BIT_0;
