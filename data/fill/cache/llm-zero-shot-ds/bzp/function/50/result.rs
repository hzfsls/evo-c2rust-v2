pub fn bzp_swap_2_elem(sort_block: &mut [i32], l_pos: i32, r_pos: i32) {
    let value = sort_block[l_pos as usize];
    sort_block[l_pos as usize] = sort_block[r_pos as usize];
    sort_block[r_pos as usize] = value;
}
