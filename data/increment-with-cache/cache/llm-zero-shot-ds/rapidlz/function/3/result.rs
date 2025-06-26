pub fn rapidlz_hash4_put_pos(pos: u32, hash_value: u32, hash_table: &mut [u32]) {
    hash_table[hash_value as usize] = pos;
}
