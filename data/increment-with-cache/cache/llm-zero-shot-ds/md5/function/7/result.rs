pub fn vos_md5_calc(output: &mut [u8], input: &[u8], input_len: u32) {
    vos_md5_calc_ex(output, MD5_DIGEST_LEN, input, input_len);
}
