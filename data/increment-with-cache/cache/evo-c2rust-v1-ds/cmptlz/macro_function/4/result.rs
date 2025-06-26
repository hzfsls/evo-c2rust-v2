macro_rules! CMPT_HASH_UPDATE { ($mf:expr, $hash2Value:expr, $hash3Value:expr, $hashValue:expr, $pos:expr) =>
    {
        $mf.hash[$hash2Value] = $pos;
        $mf.hash[CMPTLZ_FIX_3_HASH!() + $hash3Value] = $pos;
        $mf.hash[CMPTLZ_FIX_4_HASH!() + $hashValue] = $pos;
    }
}
pub(crate) use CMPT_HASH_UPDATE;
