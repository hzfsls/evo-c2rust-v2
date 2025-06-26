macro_rules! CMPTLZ_NORMAL_BIT_DEC {
    ($probLit:expr, $range:expr, $rangeCode:expr, $rangeBound:expr, $decSym:expr) => {
        $rangeBound = ($range >> CMPTLZ_PROB_LG_BIT!()) * (*$probLit);
        if $rangeCode < $rangeBound {
            CMPTLZ_RANGE_UPDATE_0!($probLit, $range, $rangeBound);
            $decSym = $decSym << 1;
        } else {
            CMPTLZ_RANGE_UPDATE_1!($probLit, $range, $rangeCode, $rangeBound);
            $decSym = ($decSym << 1) + 1;
        }
    };
}
pub(crate) use CMPTLZ_NORMAL_BIT_DEC;
