macro_rules! RAPIDLZ_READ_OPTIONAL_LENGTH {
    ($len:expr, $srcCurr:expr, $srcEnd:expr, $temp:expr) => {
        if RAPIDLZ_LIKELY!($srcCurr < $srcEnd) {
            $temp = *$srcCurr.plus_plus();
            $len += $temp;
        }
        while ($temp == RAPIDLZ_MAX_BYTE_VALUE!() && $srcCurr < $srcEnd) {
            $temp = *$srcCurr.plus_plus();
            $len += $temp;
        }
    };
}
pub(crate) use RAPIDLZ_READ_OPTIONAL_LENGTH;
