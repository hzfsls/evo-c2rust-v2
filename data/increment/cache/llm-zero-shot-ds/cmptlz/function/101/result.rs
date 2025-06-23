pub unsafe fn cmpt_lz_dec_get_probs_init(dec_ctx: *mut CmptLzDecCtx) {
    let num_probs = cmpt_lz_get_num_probs(&(*dec_ctx).prop);
    let dec_probs = (*dec_ctx).probs;

    for idx in 0..num_probs {
        *dec_probs.add(idx) = CMPTLZ_PROB_LG >> 1;
    }
    (*dec_ctx).state = 0;
}
