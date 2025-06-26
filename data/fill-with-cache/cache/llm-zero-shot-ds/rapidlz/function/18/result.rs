fn rapidlz_calc_hash_value(src_curr: &[u8], hash_type: u8, hash_bits: u8) -> u32 {
    if hash_type == 5 {
        let value = u64::from_le_bytes([
            src_curr[0], src_curr[1], src_curr[2], src_curr[3],
            src_curr[4], src_curr[5], src_curr[6], src_curr[7],
        ]);
        ((value << 24).wrapping_mul(11400714819323198485) >> (64 - hash_bits)) as u32
    } else {
        let value = u32::from_le_bytes([src_curr[0], src_curr[1], src_curr[2], src_curr[3]]);
        (value.wrapping_mul(2654435769) >> (32 - hash_bits)) as u32
    }
}
