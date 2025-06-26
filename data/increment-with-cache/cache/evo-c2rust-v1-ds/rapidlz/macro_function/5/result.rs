macro_rules! RAPIDLZ_FAST_SAFE_COPY_BY_BYTES {
    ($curDest:expr, $matchSrc:expr, $len:expr) => {
        while $len > 2 {
            *$curDest.plus_plus() = *$matchSrc.plus_plus();
            *$curDest.plus_plus() = *$matchSrc.plus_plus();
            *$curDest.plus_plus() = *$matchSrc.plus_plus();
            $len -= 3;
        }
        if $len > 0 {
            *$curDest.plus_plus() = *$matchSrc.plus_plus();
            if $len > 1 {
                *$curDest.plus_plus() = *$matchSrc.plus_plus();
            }
        }
    }
}
pub(crate) use RAPIDLZ_FAST_SAFE_COPY_BY_BYTES;
