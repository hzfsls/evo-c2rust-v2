macro_rules! cmptlz_normal_bit_dec {
    ($probLit:expr, $range:expr, $rangeCode:expr, $rangeBound:expr, $decSym:expr) => {
        {
            $rangeBound = (($range) >> $crate::CMPTLZ_PROB_LG_BIT) * (*$probLit);
            if ($rangeCode < $rangeBound) {
                $crate::CMPTLZ_RANGE_UPDATE_0!($probLit, $range, $rangeBound);
                $decSym = ($decSym << 1);
            } else {
                $crate::CMPTLZ_RANGE_UPDATE_1!($probLit, $range, $rangeCode, $rangeBound);
                $decSym = ($decSym << 1) + 1;
            }
        }
    };
}

pub(crate) use cmptlz_normal_bit_dec;
