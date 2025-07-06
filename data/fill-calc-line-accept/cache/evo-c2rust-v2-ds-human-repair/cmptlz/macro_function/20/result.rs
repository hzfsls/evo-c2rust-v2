macro_rules! CMPTLZ_MATCH_BIT_TRY_DEC {
    ($range:expr, $rangeCode:expr, $rangeBound:expr, $decSym:expr, $probSym:expr, $offset:expr, $bit:expr) => {
        $rangeBound = ($range >> CMPTLZ_PROB_LG_BIT!()) * (*$probSym as u32);
        if $rangeCode < $rangeBound {
            CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT0!($range, $rangeBound);
            $decSym = $decSym << 1;
            $offset ^= $bit;
        } else {
            CMPTLZ_RANGE_UPDATE_AFTER_DEC_BIT1!($range, $rangeCode, $rangeBound);
            $decSym = ($decSym << 1) + 1;
        }
    };
}
pub(crate) use CMPTLZ_MATCH_BIT_TRY_DEC;
