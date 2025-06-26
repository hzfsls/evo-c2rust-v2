pub fn bzp_num_encode(mtf: &mut BzpMtfInfo, num: i32) {
    let mut num = num << 1;

    loop {
        num >>= 1;
        num -= 1;
        if num & 1 != 0 {
            mtf.mtf_v[mtf.n_mtf as usize] = BZP_MTF_ENCODE1;
            mtf.mtf_freq[BZP_MTF_ENCODE1 as usize] += 1;
        } else {
            mtf.mtf_v[mtf.n_mtf as usize] = BZP_MTF_ENCODE0;
            mtf.mtf_freq[BZP_MTF_ENCODE0 as usize] += 1;
        }
        mtf.n_mtf += 1;
        if num < BZP_MTF_ENCODE_BASE {
            break;
        }
    }
}
