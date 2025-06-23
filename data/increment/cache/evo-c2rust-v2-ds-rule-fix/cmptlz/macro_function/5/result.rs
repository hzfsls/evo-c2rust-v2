macro_rules! CMPT_HASH_FIND_2_BYTES {
    ($mf:expr, $delta2:expr, $longestLen:expr, $matchesCount:expr, $cur:expr, $matches:expr) => {
        if $delta2 < $mf.cycleSize && *($cur - $delta2) == *$cur {
            $longestLen = CMPT_MF_MATCH_2_BYTES!();
            $matches[0].len = CMPT_MF_MATCH_2_BYTES!();
            $matches[0].dist = $delta2 - 1;
            $matchesCount = 1;
        }
    }
}
pub(crate) use CMPT_HASH_FIND_2_BYTES;
