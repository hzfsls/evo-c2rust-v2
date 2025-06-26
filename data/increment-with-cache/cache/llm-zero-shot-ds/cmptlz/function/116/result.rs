pub unsafe fn cmpt_lz_dec_free_probs(dec_ctx: &mut CmptLzDecCtx, mem_hook: &mut CmptLzMemHook) {
    if !dec_ctx.probs.is_null() {
        cmpt_lz_dec_mem_free(mem_hook, CMPTLZ_PROB_HANDLE, dec_ctx.probs);
        dec_ctx.probs = std::ptr::null_mut();
    }
}
