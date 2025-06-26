macro_rules! CMPT_HASH_MASK_CALC { ($hashMask:expr) =>
    {
        $hashMask |= $hashMask >> 1;
        $hashMask |= $hashMask >> 2;
        $hashMask |= $hashMask >> 4;
        $hashMask |= $hashMask >> 8;
        $hashMask >>= 1;
        $hashMask |= 0xFFFF;
        if $hashMask > (1 << 24)
        {
            $hashMask >>= 1;
        }
    }
}
pub(crate) use CMPT_HASH_MASK_CALC;
