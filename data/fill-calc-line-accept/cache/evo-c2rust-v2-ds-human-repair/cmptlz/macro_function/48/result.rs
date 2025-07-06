macro_rules! CMPT_RC_BIT_1 { ($rcCtx:expr, $prob:expr, $newBound:expr, $range:expr, $bit0Prob:expr) =>
    {
        $range -= $newBound;
        $rcCtx.low += $newBound as u64;
        *$prob = ($bit0Prob - ($bit0Prob >> 5)).cast();
    }
}
pub(crate) use CMPT_RC_BIT_1;
