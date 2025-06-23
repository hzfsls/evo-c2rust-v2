macro_rules! CMPT_MF_MOVE_POS {
    ($mf:expr) => {
        {
            $mf.readPos.plus_plus();
            $mf.cyclePos.plus_plus();
            $mf.cyclePos = if $mf.cyclePos == $mf.cycleSize {
                0
            } else {
                $mf.cyclePos
            };
            if CMPTLZ_UNLIKELY!($mf.readPos + $mf.offset == CMPTLZ_UINT32_MAX!()) {
                CmptMfMovePos($mf.cast());
            }
        }
    }
}
pub(crate) use CMPT_MF_MOVE_POS;