macro_rules! RAPIDLZ_SAFE_COPY_TILL_END {
    ($curSrc:expr, $leftSrcSize:expr, $curDest:expr, $destEnd:expr, $len:expr) => {
        RAPIDLZ_SAFE_LIT_COPY!($curSrc, $leftSrcSize, $curDest, $destEnd, $len);
        RAPIDLZ_POSITION_UPDATE!($curSrc, $curDest, $len);
        if $leftSrcSize == $len {
            return $curDest - c_ref!($dest).cast::<Ptr<u8>>();
        }
    }
}
pub(crate) use RAPIDLZ_SAFE_COPY_TILL_END;
