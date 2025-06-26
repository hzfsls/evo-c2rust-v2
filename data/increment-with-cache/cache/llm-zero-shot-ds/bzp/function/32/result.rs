pub fn bzp_map_input_char(mtf: &mut BzpMtfInfo, list: &mut [u8], len_list: i32) {
    if BZP_ASCII_SIZE > len_list {
        return;
    }
    for i in 0..BZP_ASCII_SIZE {
        if mtf.in_use[i] {
            list[mtf.n_use as usize] = i as u8;
            mtf.n_use += 1;
        }
    }
}
