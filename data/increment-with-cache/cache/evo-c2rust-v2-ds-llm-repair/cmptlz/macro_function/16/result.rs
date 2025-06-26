macro_rules! CMPT_RC_BIT_0 { ($prob:expr, $newBound:expr, $range:expr, $bit0Prob:expr) =>
    {
        $range = $newBound;
        *$prob = ($bit0Prob + ((CMPTLZ_PROB_MAX_NUM - $bit0Prob) >> 5)) as CmptlzProb;
    }
}
pub(crate) use CMPT_RC_BIT_0;
