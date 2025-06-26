macro_rules! SAFE_COPY_MATCH { ($dstCurr:expr, $matchSrc:expr, $matchLength:expr) =>
    {
        let mut matchLength = $matchLength;
        while matchLength > 0
        {
            matchLength -= 1;
            *$dstCurr.plus_plus() = *$matchSrc.plus_plus();
        }
    }
}
pub(crate) use SAFE_COPY_MATCH;
