macro_rules! CMPT_LIT_PROB_GET {
    ($encCtx:expr, $litProb:expr, $pos:expr, $prevByte:expr) => {
        $litProb + (3 * (((($pos << 8) + $prevByte) & $encCtx.lpMask) << $encCtx.litMarcov.lcBits)) as u32
    };
}

pub(crate) use CMPT_LIT_PROB_GET;
