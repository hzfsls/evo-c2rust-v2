fn cmpt_lz_range_code_init(dec_ctx: &mut CmptLzDecCtx) {
    let range_code = (u32::from(dec_ctx.temp_buf[1])) << 24
        | (u32::from(dec_ctx.temp_buf[2])) << 16
        | (u32::from(dec_ctx.temp_buf[3])) << 8
        | u32::from(dec_ctx.temp_buf[4]);
    dec_ctx.code = range_code;
    dec_ctx.range = 0xFFFFFFFF;
}
