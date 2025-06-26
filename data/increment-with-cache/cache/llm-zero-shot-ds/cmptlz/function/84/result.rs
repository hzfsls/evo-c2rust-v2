pub fn cmpt_lz_get_ailgn_prob(probs_matrix: *mut CmptLzDecProb) -> *mut CmptLzDecProb {
    unsafe {
        probs_matrix.add(CMPTLZ_ALIGN)
    }
}
