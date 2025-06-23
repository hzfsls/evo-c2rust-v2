macro_rules! CMPT_RC_BIT_0_PROCESS { ($rcCtx:expr, $prob:expr, $newBound:expr, $range:expr, $bit0Prob:expr, $shiftRes:expr) => 
    {
        CMPT_RC_BIT_0!($prob, $newBound, $range, $bit0Prob);
        CMPT_RC_NORMALIZE!($rcCtx, $range, $shiftRes);
    }
}
pub(crate) use CMPT_RC_BIT_0_PROCESS;