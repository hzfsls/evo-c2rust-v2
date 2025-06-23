macro_rules! RAPIDLZ_SAFE_LIT_COPY {
    ($curSrc:expr, $leftSrcSize:expr, $curDest:expr, $destEnd:expr, $litLen:expr) => {
        if $litLen > $leftSrcSize || unsafe { libc::memmove($curDest as *mut libc::c_void, $curSrc as *const libc::c_void, $litLen) }.is_null() {
            RAPIDLZ_LOG!(RAPIDLZ_DST_SIZE_SMALL, "litLen:{} dstEnd - dst:{}\n", $litLen, $leftSrcSize);
            return RAPIDLZ_ERROR_OUTPUT;
        }
    };
}

pub(crate) use RAPIDLZ_SAFE_LIT_COPY;
