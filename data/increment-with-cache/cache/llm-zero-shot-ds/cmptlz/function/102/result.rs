#[inline]
fn cmpt_lz_range_code_init(dec_ctx: &mut CmptLzDecCtx) {
    let mut range_code = (dec_ctx.temp_buf[1] as u32) << 24;
    range_code |= (dec_ctx.temp_buf[2] as u32) << 16;
    range_code |= (dec_ctx.temp_buf[3] as u32) << 8;
    range_code |= dec_ctx.temp_buf[4] as u32;
    dec_ctx.code = range_code;
    dec_ctx.range = 0xFFFFFFFF;
}
