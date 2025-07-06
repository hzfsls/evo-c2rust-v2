macro_rules! CMPT_HASH_4_CALC {
    ($mf:expr, $cur:expr, $temp:expr, $hash2Value:expr, $hash3Value:expr, $hashValue:expr) => {
        $temp = $mf.hashRootTable[$cur[0]] ^ $cur[1] as u32;
        $hash2Value = $temp & CMPTLZ_HASH_2_MASK!();
        $hash3Value = ($temp ^ (($cur[2] as u32) << 8)) & CMPTLZ_HASH_3_MASK!();
        $hashValue = ($temp ^ (($cur[2] as u32) << 8) ^ ($mf.hashRootTable[$cur[3]] << 5)) & $mf.hashMask;
    }
}
pub(crate) use CMPT_HASH_4_CALC;
