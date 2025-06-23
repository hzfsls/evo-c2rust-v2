pub struct BzpMtfInfo {
    n_use: i32,
    n_mtf: i32,
    block: *mut u8,
    map: *mut u8,
    in_use: *mut u8,
}

pub fn bzp_mtf_reset(mtf: &mut BzpMtfInfo) {
    mtf.n_use = 0;
    mtf.n_mtf = 0;
    mtf.block = std::ptr::null_mut();
    mtf.map = std::ptr::null_mut();
    mtf.in_use = std::ptr::null_mut();
}
