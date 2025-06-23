macro_rules! CMPT_HASH_FIND_3_BYTES {
    ($mf:expr, $delta2:expr, $delta3:expr, $longestLen:expr, $matchesCount:expr, $cur:expr, $matches:expr) => {
        if $delta3 != $delta2 && $delta3 < $mf.cycleSize && *($cur - $delta3) == *$cur {
            $longestLen = CMPT_MF_MATCH_3_BYTES!();
            $matches[$matchesCount].dist = $delta3 - 1;
            $matchesCount += 1;
            $delta2 = $delta3;
        }
    }
}
pub(crate) use CMPT_HASH_FIND_3_BYTES;