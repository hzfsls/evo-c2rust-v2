macro_rules! cmptlz_is_the_bit_0 {
    ($probSlot:expr, $range:expr, $rangeCode:expr, $rangeBound:expr) => {
        $rangeBound = ($range >> CMPTLZ_PROB_LG_BIT) * (*$probSlot);
        $rangeCode < $rangeBound
    };
}

pub(crate) use cmptlz_is_the_bit_0;
