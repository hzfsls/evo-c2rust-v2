#[inline]
fn rapidlz_copy_literals_fast(src: &[u8], dst: &mut [u8], length: u32) {
    if likely!(length <= RAPIDLZ_SIXTEEN_BYTE) {
        rapidlz_copy_16_byte(dst, src);
        return;
    }

    rapidlz_wild_copy_16(src, dst, dst.len() + length as usize);
}
