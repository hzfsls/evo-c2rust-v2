macro_rules! SAFE_COPY_MATCH {
    ($dstCurr:expr, $matchSrc:expr, $matchLength:expr) => {
        {
            let mut length = $matchLength;
            let mut dst = $dstCurr;
            let mut src = $matchSrc;
            while length > 0 {
                *dst = *src;
                dst = dst.wrapping_add(1);
                src = src.wrapping_add(1);
                length -= 1;
            }
        }
    };
}

pub(crate) use SAFE_COPY_MATCH;
