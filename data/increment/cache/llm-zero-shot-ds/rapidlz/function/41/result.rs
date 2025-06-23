#[inline]
fn rapidlz_calc_hash_value(src_curr: &[u8], hash_type: u8, hash_bits: u8) -> u32 {
    if hash_type == 5 {
        let value = u64::from_le_bytes(src_curr[..8].try_into().unwrap());
        ((value.wrapping_shl(24).wrapping_mul(11400714819323198485u64)) >> (64 - hash_bits)) as u32
    } else {
        let value = u32::from_le_bytes(src_curr[..4].try_into().unwrap());
        (value.wrapping_mul(2654435769u32) >> (32 - hash_bits)) as u32
    }
}
