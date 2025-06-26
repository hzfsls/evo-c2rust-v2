macro_rules! RAPIDLZ_GET_MATCH_LEN {
    ($len:expr, $curSrc:expr, $srcEnd:expr, $temp:expr) => {
        if $len < $crate::RAPIDLZ_MAX_4BIT_VALUE {
            $len += 4;
        } else {
            $crate::RAPIDLZ_READ_OPTIONAL_LENGTH!($len, $curSrc, $srcEnd, $temp);
            $len += 4;
        }
    };
}

pub(crate) use RAPIDLZ_GET_MATCH_LEN;
