macro_rules! CMPT_MF_MOVE_POS {
    ($mf:expr) => {
        {
            $mf.readPos += 1;
            $mf.cyclePos += 1;
            $mf.cyclePos = if $mf.cyclePos == $mf.cycleSize { 0 } else { $mf.cyclePos };
            if crate::CMPTLZ_UNLIKELY($mf.readPos + $mf.offset == crate::CMPTLZ_UINT32_MAX) {
                crate::CmptMfMovePos($mf);
            }
        }
    };
}

pub(crate) use CMPT_MF_MOVE_POS;
