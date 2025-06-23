macro_rules! RAPIDLZ_READ_OPTIONAL_LENGTH {
    ($len:expr, $srcCurr:expr, $srcEnd:expr, $temp:expr) => {
        if crate::likely($srcCurr < $srcEnd) {
            $temp = *$srcCurr;
            $srcCurr = $srcCurr.wrapping_add(1);
            $len += $temp;
        }
        while ($temp == crate::RAPIDLZ_MAX_BYTE_VALUE) && ($srcCurr < $srcEnd) {
            $temp = *$srcCurr;
            $srcCurr = $srcCurr.wrapping_add(1);
            $len += $temp;
        }
    };
}

pub(crate) use RAPIDLZ_READ_OPTIONAL_LENGTH;
