macro_rules! SAFE_COPY_MATCH {
    ($dstCurr:expr, $matchSrc:expr, $matchLength:expr) => {
        while {
            let tmp = $matchLength;
            $matchLength -= 1;
            tmp
        } != 0
        {
            *$dstCurr = *$matchSrc;
            $dstCurr += 1;
            $matchSrc += 1;
        }
    };
}
pub(crate) use SAFE_COPY_MATCH;