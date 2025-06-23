static fn cmpt_lz_get_is_rep_g0_long_prob(probs_matrix: *mut CmptLzDecProb) -> *mut CmptLzDecProb {
    unsafe { probs_matrix.offset(CMPTLZ_REP0_LONG as isize) }
}
