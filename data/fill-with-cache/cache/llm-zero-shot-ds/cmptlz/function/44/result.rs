fn cmpt_lz_dec_rem_write_in_dict(dec_ctx: &mut CmptLzDecCtx, dic_pos_limit: usize) {
    let dict_pos = dec_ctx.dict_pos;
    let mut remain_dec_len = dec_ctx.remain_len;
    let dict_buf_size = dec_ctx.dict_buf_size;
    let remain_dic_len = dic_pos_limit - dict_pos;
    
    if remain_dic_len < remain_dec_len {
        remain_dec_len = remain_dic_len;
    }
    
    if remain_dec_len == 0 {
        return;
    }
    
    dec_ctx.processed_pos += remain_dec_len as u32;
    dec_ctx.remain_len -= remain_dec_len as u32;
    
    let dict = &mut dec_ctx.dict;
    let rep0 = dec_ctx.reps[0];
    
    let mut current_pos = dict_pos;
    let mut remaining = remain_dec_len;
    
    while remaining != 0 {
        remaining -= 1;
        let src_pos = current_pos - rep0 + if current_pos < rep0 { dict_buf_size } else { 0 };
        dict[current_pos] = dict[src_pos];
        current_pos += 1;
    }
    
    dec_ctx.dict_pos = current_pos;
    cmpt_lz_dec_check_dict_size_update(dec_ctx);
}
