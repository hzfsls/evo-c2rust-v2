fn rapidlz_get_pos_on_table(hash_value: u32, hash_table: &[u8], hash_type: u8) -> u32 {
    match hash_type {
        4 => {
            let offset = hash_value as usize * 2;
            if offset + 1 < hash_table.len() {
                u16::from_le_bytes([hash_table[offset], hash_table[offset + 1]]) as u32
            } else {
                0
            }
        }
        5 => {
            let offset = hash_value as usize * 4;
            if offset + 3 < hash_table.len() {
                u32::from_le_bytes([
                    hash_table[offset],
                    hash_table[offset + 1],
                    hash_table[offset + 2],
                    hash_table[offset + 3],
                ])
            } else {
                0
            }
        }
        _ => 0,
    }
}
