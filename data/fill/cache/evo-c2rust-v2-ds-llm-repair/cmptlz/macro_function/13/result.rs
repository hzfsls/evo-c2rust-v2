macro_rules! CMPTLZ_DIST_BIT_DEC {
    ($probDist:expr, $probSlot:expr, $range:expr, $rangeCode:expr, $rangeBound:expr, $decDist:expr, $decBit:expr) => {
        $probDist = $probSlot + $decDist;
        $rangeBound = ($range >> CMPTLZ_PROB_LG_BIT!()) * (*$probDist);
        if $rangeCode < $rangeBound {
            CMPTLZ_RANGE_UPDATE_0!($probDist, $range, $rangeBound);
            $decDist += $decBit;
        } else {
            CMPTLZ_RANGE_UPDATE_1!($probDist, $range, $rangeCode, $rangeBound);
            $decDist += $decBit * 2;
        }
    };
}
pub(crate) use CMPTLZ_DIST_BIT_DEC;
