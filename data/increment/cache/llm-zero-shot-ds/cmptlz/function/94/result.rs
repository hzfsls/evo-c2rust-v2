pub fn cmpt_lz_short_rep_dec(dec_ctx: &mut CmptLzDecCtx) {
    let rep0 = dec_ctx.reps[0];
    let dict = &mut dec_ctx.dict;
    let dict_pos = dec_ctx.dict_pos;
    let dict_buf_size = dec_ctx.dict_buf_size;

    let offset = if dict_pos < rep0 {
        dict_buf_size
    } else {
        0
    };
    dict[dict_pos] = dict[dict_pos - rep0 + offset];
    
    dec_ctx.dict_pos += 1;
    dec_ctx.processed_pos += 1;
    
    if dec_ctx.state < CMPTLZ_LIT_STATES {
        dec_ctx.state = 9;
    } else {
        dec_ctx.state = 11;
    }
}
