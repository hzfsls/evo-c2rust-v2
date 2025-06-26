fn rapidlz_copy_8_byte(dst: &mut [u8], src: &[u8]) {
    assert!(dst.len() >= 8 && src.len() >= 8, "Source and destination must be at least 8 bytes");
    let src_val = u64::from_le_bytes(src[..8].try_into().unwrap());
    dst[..8].copy_from_slice(&src_val.to_le_bytes());
}
