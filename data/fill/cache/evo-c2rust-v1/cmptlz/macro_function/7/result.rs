macro_rules! CMPTLZ_RANGE_NORMALIZE {
    ($range:expr, $rangeCode:expr, $bufToDec:expr) => {
        if $range < CMPTLZ_RANGE_DOWN_LIMIT!() {
            $range <<= CMPTLZ_ONE_BYTE_WIDTH!();
            $rangeCode <<= CMPTLZ_ONE_BYTE_WIDTH!();
            $rangeCode |= $bufToDec.plus_plus();
        }
    }
}
pub(crate) use CMPTLZ_RANGE_NORMALIZE;
