fn cmpt_lz_dec_check_dict_size_update(dec_ctx: &mut CmptLzDecCtx) {
    if dec_ctx.check_dic_size == 0 && dec_ctx.processed_pos >= dec_ctx.prop.dic_size {
        dec_ctx.check_dic_size = dec_ctx.prop.dic_size;
    }
}
