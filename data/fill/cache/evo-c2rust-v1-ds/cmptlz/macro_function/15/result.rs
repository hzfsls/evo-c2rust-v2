macro_rules! CMPTLZ_POSSLOT_BIT_DEC {
    ($probSlot:expr, $range:expr, $rangeCode:expr, $rangeBound:expr, $decSym:expr, $bufToDec:expr) => {
        CMPTLZ_NORMAL_BIT_DEC!($probSlot, $range, $rangeCode, $rangeBound, $decSym);
        CMPTLZ_RANGE_NORMALIZE!($range, $rangeCode, $bufToDec);
    }
}
pub(crate) use CMPTLZ_POSSLOT_BIT_DEC;
