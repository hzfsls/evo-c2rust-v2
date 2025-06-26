macro_rules! CMPT_LIT_PROB_GET {
    ($encCtx:expr, $litProb:expr, $pos:expr, $prevByte:expr) => {
        $litProb + (3 as u32) * (((($pos << 8) + $prevByte) & $encCtx.lpMask) << $encCtx.litMarcov.lcBits)
    }
}
pub(crate) use CMPT_LIT_PROB_GET;
