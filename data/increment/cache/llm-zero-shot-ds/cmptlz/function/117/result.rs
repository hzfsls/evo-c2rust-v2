pub unsafe fn cmpt_lz_free_dict(dec_ctx: &mut CmptLzDecCtx, mem_hook: &mut CmptLzMemHook) {
    if !dec_ctx.dict.is_null() {
        cmpt_lz_dec_mem_free(mem_hook, CMPTLZ_DICT_HANDLE, dec_ctx.dict);
        dec_ctx.dict = std::ptr::null_mut();
    }
}
