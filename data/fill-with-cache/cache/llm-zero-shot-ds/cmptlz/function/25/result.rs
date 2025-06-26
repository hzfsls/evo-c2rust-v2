fn cmpt_lz_get_match_len_coder_prob(probs_matrix: &mut [CmptLzDecProb]) -> &mut CmptLzDecProb {
    &mut probs_matrix[CMPTLZ_MATCH_LEN_CODER]
}
