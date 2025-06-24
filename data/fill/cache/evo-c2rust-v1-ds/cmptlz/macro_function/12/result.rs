macro_rules! CMPTLZ_MATCH_BIT_DEC {
    ($probSlot:expr, $range:expr, $rangeCode:expr, $rangeBound:expr, $decSym:expr, $matchSym:expr, $offset:expr, $bit:expr, $bufToDec:expr) => {
        $matchSym <<= 1;
        $bit = $offset;
        $offset &= $matchSym;
        $probLit = $probSlot + ($offset + $bit + $decSym);
        $rangeBound = ($range >> CMPTLZ_PROB_LG_BIT!()) * (*$probLit);
        if $rangeCode < $rangeBound {
            CMPTLZ_RANGE_UPDATE_0!($probLit, $range, $rangeBound);
            $decSym = ($decSym << 1);
            $offset ^= $bit;
        } else {
            CMPTLZ_RANGE_UPDATE_1!($probLit, $range, $rangeCode, $rangeBound);
            $decSym = ($decSym << 1) + 1;
        }
        CMPTLZ_RANGE_NORMALIZE!($range, $rangeCode, $bufToDec);
    }
}
pub(crate) use CMPTLZ_MATCH_BIT_DEC;
