pub fn vos_sha256_calc(puc_input: &[u8], ui_input_len: u32, puc_output: &mut [u8], ui_output_len: u32) {
    let mut st_ctx = VOS_SHA256_CTX::new();
    vos_sha256_begin(&mut st_ctx);
    vos_sha256_hash(puc_input, ui_input_len, &mut st_ctx);
    vos_sha256_end(puc_output, ui_output_len, &mut st_ctx);
}
