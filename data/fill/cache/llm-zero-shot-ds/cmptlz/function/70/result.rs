pub fn cmptlz_set_param(enc_ctx: &mut CmptLzEncCtx, props: &CmptlzEncParam) {
    let mut param = *props;
    cmptlz_param_normalize(&mut param);
    enc_ctx.dic_size = param.dict_size;
    enc_ctx.num_fast_bytes = param.fast_bytes;
    enc_ctx.lit_ctx = param.lit_ctx;
    enc_ctx.lit_pos = param.lit_pos;
    enc_ctx.pos_bits = param.pos_bits;
    
    let mut i = 7;
    while i < 32 {
        if enc_ctx.dic_size <= (1 << i) {
            break;
        }
        i += 1;
    }
    enc_ctx.dist_table_size = i * 2;
}
