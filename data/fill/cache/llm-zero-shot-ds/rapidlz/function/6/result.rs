fn rapidlz_copy_16_byte(dst: &mut [u8], src: &[u8]) {
    assert!(dst.len() >= 16);
    assert!(src.len() >= 16);
    
    // Copy first 8 bytes
    let first_chunk = u64::from_le_bytes(src[0..8].try_into().unwrap());
    dst[0..8].copy_from_slice(&first_chunk.to_le_bytes());
    
    // Copy next 8 bytes
    let second_chunk = u64::from_le_bytes(src[8..16].try_into().unwrap());
    dst[8..16].copy_from_slice(&second_chunk.to_le_bytes());
}
