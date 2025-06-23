macro_rules! RAPIDLZ_CONTINUE_IF_NOT_A_MATCH {
    ($matchOffset:expr, $prefixDictLimit:expr, $current:expr) => {
        if ($matchOffset < $prefixDictLimit) || ($matchOffset + RAPIDLZ_MAX_OFFSET!() < $current) {
            continue;
        }
    }
}
pub(crate) use RAPIDLZ_CONTINUE_IF_NOT_A_MATCH;
