fn cmpt_lz_get_literal_prob(probs_matrix: &mut CmptLzDecProb) -> &mut CmptLzDecProb {
    probs_matrix.offset(CMPTLZ_LITERAL as isize)
}
