pub fn cmpt_lz_dec_careful_process(
    dec_ctx: &mut CmptLzDecCtx,
    dic_pos_limit: usize,
    buf_limit: *const u8,
) -> i32 {
    let mut res = CMPT_OK;
    let mut buf_limit_tmp;
    let mut p_src_in;
    
    loop {
        buf_limit_tmp = buf_limit;
        p_src_in = dec_ctx.buf;
        res = cmpt_lz_try_dec_one_packet(dec_ctx, p_src_in, &mut buf_limit_tmp);
        
        if res == CMPTLZ_DEC_INPUT_EOF {
            break;
        }
        
        res = cmpt_lz_dec_direct_process(dec_ctx, dic_pos_limit, buf_limit_tmp);
        
        if res != CMPT_OK || dec_ctx.buf != buf_limit_tmp {
            return CMPT_ERROR_DATA;
        }
        
        if dec_ctx.remain_len == CMPTLZ_MATCH_MAX_LEN {
            break;
        }
        
        if dec_ctx.dict_pos >= dic_pos_limit {
            break;
        }
    }
    
    if res == CMPTLZ_DEC_INPUT_EOF && dec_ctx.buf < buf_limit {
        let remain_len = (buf_limit as usize - dec_ctx.buf as usize) as u32;
        dec_ctx.temp_buf_size = remain_len;
        
        for idx in 0..remain_len {
            dec_ctx.temp_buf[idx as usize] = dec_ctx.buf[idx as usize];
        }
    }
    
    CMPT_OK
}
