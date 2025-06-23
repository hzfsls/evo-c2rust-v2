pub fn cmpt_lz_get_is_match_prob(probs_matrix: &mut [CmptLzDecProb]) -> &mut CmptLzDecProb {
    &mut probs_matrix[CMPTLZ_IS_MATCH]
}
