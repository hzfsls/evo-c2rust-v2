macro_rules! CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT1 { ($range:expr, $rangeCode:expr, $rangeBound:expr) =>
    {
        $range -= $rangeBound;
        $rangeCode -= $rangeBound;
    }
}
pub(crate) use CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT1;
