pub fn rapidlz_compress_default(src: &[u8], dst: &mut [u8], src_size: usize, dst_size: usize) -> usize {
    rapidlz_compress(src, dst, src_size, dst_size, 1)
}
