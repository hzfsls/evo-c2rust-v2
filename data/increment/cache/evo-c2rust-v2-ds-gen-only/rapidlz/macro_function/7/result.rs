macro_rules! RAPIDLZ_LIT_AND_MATCH_COPY_END { ($curDest:expr, $matchLen:expr) => { $curDest + (1 + RAPIDLZ_LAST_LITERALS!()) + (($matchLen + 240) / RAPIDLZ_MAX_BYTE_VALUE!() } }
pub(crate) use RAPIDLZ_LIT_AND_MATCH_COPY_END;
