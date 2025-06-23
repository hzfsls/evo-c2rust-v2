static fn cmpt_lz_get_pos_slot_prob(probs_matrix: &mut CmptLzDecProb) -> &mut CmptLzDecProb {
    &mut probs_matrix[CMPTLZ_POSSLOT]
}
