macro_rules! cmptlz_match_bit_dec {
    ($probSlot:expr, $range:expr, $rangeCode:expr, $rangeBound:expr, $decSym:expr, $matchSym:expr, $offset:expr, $bit:expr, $bufToDec:expr) => {
        $matchSym <<= 1;
        $bit = $offset;
        $offset &= $matchSym;
        $probLit = $probSlot + ($offset + $bit + $decSym);
        $rangeBound = ($range >> CMPTLZ_PROB_LG_BIT) * (*$probLit);
        if $rangeCode < $rangeBound {
            cmptlz_range_update_0!($probLit, $range, $rangeBound);
            $decSym = $decSym << 1;
            $offset ^= $bit;
        } else {
            cmptlz_range_update_1!($probLit, $range, $rangeCode, $rangeBound);
            $decSym = ($decSym << 1) + 1;
        }
        cmptlz_range_normalize!($range, $rangeCode, $bufToDec);
    };
}

pub(crate) use cmptlz_match_bit_dec;
