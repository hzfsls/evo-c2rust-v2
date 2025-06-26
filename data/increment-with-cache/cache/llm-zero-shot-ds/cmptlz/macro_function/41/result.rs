macro_rules! cmptlz_len_bit_dec {
    ($probSlot:expr, $range:expr, $rangeCode:expr, $rangeBound:expr, $decSym:expr, $bufToDec:expr) => {
        cmptlz_normal_bit_dec!($probSlot, $range, $rangeCode, $rangeBound, $decSym);
        cmptlz_range_normalize!($range, $rangeCode, $bufToDec);
    };
}

pub(crate) use cmptlz_len_bit_dec;
