pub fn rapidlz_hash4_get_pos(hash_value: u32, hash_table: &[u32]) -> u32 {
    hash_table[hash_value as usize]
}
