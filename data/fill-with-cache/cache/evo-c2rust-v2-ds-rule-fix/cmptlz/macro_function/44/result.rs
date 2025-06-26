macro_rules! CMPT_RC_NORMALIZE { ($rcCtx:expr, $range:expr, $shiftRes:expr) =>
    {
        if $range < CMPT_RC_MIN_RANGE!()
        {
            $range <<= 8;
            $shiftRes = CmptRcShiftLow($rcCtx);
        }
    }
}
pub(crate) use CMPT_RC_NORMALIZE;
