pub fn rapidlz_compress_bound(src_size: usize) -> usize {
    src_size + (src_size / 100) + 16
}
