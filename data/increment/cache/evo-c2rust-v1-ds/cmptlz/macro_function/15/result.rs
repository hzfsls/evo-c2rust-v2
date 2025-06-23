macro_rules! CMPT_RC_BIT_PROCESS {
    ($rcCtx:expr, $prob:expr, $bit:expr, $bit0Prob:expr, $range:expr, $newBound:expr, $shiftRes:expr) => {
        {
            let mut mask = 0 - ($bit as u32);
            CMPT_RC_GET_NEWBOUND!($prob, $bit0Prob, $range, $newBound);
            $range &= mask;
            mask &= $newBound;
            $range -= mask;
            (*$rcCtx.lock()).low += mask;
            mask = ($bit as u32) - 1;
            $range += $newBound & mask;
            mask &= (CMPTLZ_PROB_MAX_NUM - ((1 << 5) - 1));
            mask += ((1 << 5) - 1);
            $bit0Prob += (mask - ($bit0Prob as u32)) as i32 >> 5;
            *$prob = $bit0Prob as CmptlzProb;
            CMPT_RC_NORMALIZE!($rcCtx, $range, $shiftRes);
        }
    }
}
pub(crate) use CMPT_RC_BIT_PROCESS;
