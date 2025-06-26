macro_rules! CMPTLZ_RANGE_TRY_NORMALIZE { ($range:expr, $rangeCode:expr, $bufTryDec:expr, $bufLimit:expr) =>
    {
        if $range < CMPTLZ_RANGE_DOWN_LIMIT!()
        {
            if $bufTryDec >= $bufLimit
            {
                return CMPTLZ_DEC_INPUT_EOF!();
            }
            $range <<= CMPTLZ_ONE_BYTE_WIDTH!();
            $rangeCode <<= CMPTLZ_ONE_BYTE_WIDTH!();
            $rangeCode |= *$bufTryDec.plus_plus();
        }
    }
}
pub(crate) use CMPTLZ_RANGE_TRY_NORMALIZE;
