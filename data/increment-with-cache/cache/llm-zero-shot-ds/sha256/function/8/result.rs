pub fn vos_sha256_calc(puc_input: &[u8], ui_input_len: u32, puc_output: &mut [u8], ui_output_len: u32) {
    let mut st_ctx = VOS_SHA256_CTX::new();
    
    st_ctx.begin();
    st_ctx.hash(puc_input, ui_input_len);
    st_ctx.end(puc_output, ui_output_len);
}
