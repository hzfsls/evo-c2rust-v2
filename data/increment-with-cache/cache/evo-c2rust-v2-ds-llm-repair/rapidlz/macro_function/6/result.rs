macro_rules! RAPIDLZ_LITERAL_LEN_COPY_END {
    ($curDest:expr, $len:expr) => {
        $curDest + $len + 1 + (($len + RAPIDLZ_MAX_BYTE_VALUE!() - RAPIDLZ_MAX_4BIT_VALUE!()) / RAPIDLZ_MAX_BYTE_VALUE!())
    }
}
pub(crate) use RAPIDLZ_LITERAL_LEN_COPY_END;
