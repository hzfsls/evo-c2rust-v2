pub fn bzp_swap_3_elem(sort_block: &mut [i32], l_pos: usize, e_pos: usize, r_pos: usize) {
    let value = sort_block[l_pos];
    sort_block[l_pos] = sort_block[r_pos];
    sort_block[r_pos] = sort_block[e_pos];
    sort_block[e_pos] = value;
}
