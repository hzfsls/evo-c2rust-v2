pub fn cmpt_lz_dec_init(dec_ctx: &mut CmptLzDecCtx) {
    dec_ctx.dict_pos = 0;
    dec_ctx.temp_buf_size = 0;
    dec_ctx.processed_pos = 0;
    dec_ctx.check_dic_size = 0;
    dec_ctx.remain_len = CMPTLZ_MATCH_MAX_LEN + 2;
}
