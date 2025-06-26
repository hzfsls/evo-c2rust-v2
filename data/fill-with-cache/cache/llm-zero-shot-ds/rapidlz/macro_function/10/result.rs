macro_rules! SAFE_COPY_MATCH {
    ($dstCurr:expr, $matchSrc:expr, $matchLength:expr) => {
        let mut match_length = $matchLength;
        while match_length > 0 {
            *$dstCurr = *$matchSrc;
            $dstCurr = $dstCurr.wrapping_add(1);
            $matchSrc = $matchSrc.wrapping_add(1);
            match_length -= 1;
        }
    };
}

pub(crate) use SAFE_COPY_MATCH;
