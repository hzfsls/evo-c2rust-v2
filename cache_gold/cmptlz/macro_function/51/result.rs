macro_rules! CMPT_LIT_PROB_GET {
    ($encCtx:expr, $litProb:expr, $pos:expr, $prevByte:expr) => {
        $litProb + (3 * (((($pos << 8) + $prevByte as u32) & $encCtx.lpMask as u32) << $encCtx.litMarcov.lcBits))
    }
}
pub(crate) use CMPT_LIT_PROB_GET;