fn cmpt_lz_get_rep_len_coder_prob(probs_matrix: &mut [CmptLzDecProb]) -> &mut CmptLzDecProb {
    &mut probs_matrix[CMPTLZ_REP_LEN_CODER]
}
