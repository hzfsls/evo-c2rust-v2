fn cmpt_lz_get_probs_matrix(dec_ctx: &CmptLzDecCtx) -> &CmptLzDecProb {
    &dec_ctx.probs_plus_1664
}
