macro_rules! CMPT_HASH_4_CALC {
    ($mf:expr, $cur:expr, $temp:expr, $hash2Value:expr, $hash3Value:expr, $hashValue:expr) => {
        $temp = $mf.hashRootTable[$cur[0] as usize] ^ $cur[1];
        $hash2Value = $temp & CMPTLZ_HASH_2_MASK;
        $hash3Value = ($temp ^ ((u32::from($cur[2])) << 8)) & CMPTLZ_HASH_3_MASK;
        $hashValue = ($temp ^ ((u32::from($cur[2])) << 8) ^ ($mf.hashRootTable[$cur[3] as usize] << 5)) & $mf.hashMask;
    };
}

pub(crate) use CMPT_HASH_4_CALC;
