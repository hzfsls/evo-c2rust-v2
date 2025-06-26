macro_rules! RAPIDLZ_DICT_FAST_COPY_AVAIL {
    ($curSrc:expr, $len:expr, $srcEndFast:expr, $curDest:expr, $destEndFast:expr) => {
        ($curSrc + $len <= $srcEndFast) && ($curDest + $len <= $destEndFast)
    }
}
pub(crate) use RAPIDLZ_DICT_FAST_COPY_AVAIL;
