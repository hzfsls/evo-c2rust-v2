macro_rules! CMPT_RC_BIT_PROCESS {
    ($rcCtx:expr, $prob:expr, $bit:expr, $bit0Prob:expr, $range:expr, $newBound:expr, $shiftRes:expr) => {
        {
            let mask = 0u32.wrapping_sub($bit as u32);
            CMPT_RC_GET_NEWBOUND!($prob, $bit0Prob, $range, $newBound);
            $range &= mask;
            let mask = mask & $newBound;
            $range -= mask;
            $rcCtx.low += mask;
            let mask = ($bit as u32).wrapping_sub(1);
            $range += $newBound & mask;
            let mask = mask & (CMPTLZ_PROB_MAX_NUM - ((1 << 5) - 1));
            let mask = mask + ((1 << 5) - 1);
            $bit0Prob += (mask as i32 - $bit0Prob) >> 5;
            *$prob = $bit0Prob as CmptlzProb;
            CMPT_RC_NORMALIZE!($rcCtx, $range, $shiftRes);
        }
    };
}

pub(crate) use CMPT_RC_BIT_PROCESS;
