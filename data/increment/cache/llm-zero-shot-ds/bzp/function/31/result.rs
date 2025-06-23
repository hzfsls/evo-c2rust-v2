pub fn bzp_mtf_reset(mtf: &mut BzpMtfInfo) {
    mtf.n_use = 0;
    mtf.n_mtf = 0;
    mtf.block = std::ptr::null_mut();
    mtf.map = std::ptr::null_mut();
    mtf.in_use = std::ptr::null_mut();
}
