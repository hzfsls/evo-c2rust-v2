macro_rules! RAPIDLZ_SAFE_COPY_TILL_END {
    ($curSrc:expr, $leftSrcSize:expr, $curDest:expr, $destEnd:expr, $len:expr) => {
        $crate::RAPIDLZ_SAFE_LIT_COPY!($curSrc, $leftSrcSize, $curDest, $destEnd, $len);
        $crate::RAPIDLZ_POSITION_UPDATE!($curSrc, $curDest, $len);
        if $leftSrcSize == $len {
            return $curDest - ($dest as *const u8) as usize;
        }
    };
}

pub(crate) use RAPIDLZ_SAFE_COPY_TILL_END;
