pub fn rapidlz_hash4_calc_value(cur_src: &[u8]) -> u32 {
    // Assuming RAPIDLZ_READ32BIT reads a 32-bit little-endian value from cur_src
    let value = u32::from_le_bytes([cur_src[0], cur_src[1], cur_src[2], cur_src[3]]);
    // Constants would be defined elsewhere in the Rust code
    (value.wrapping_mul(RAPIDLZ_GOLDEN_SECTION_PRIME)) >> RAPIDLZ_STREAM_HASH_BITS
}
