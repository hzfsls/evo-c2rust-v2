fn rapidlz_get_pos_on_table(hash_value: u32, hash_table: &mut [u8], hash_type: u8) -> u32 {
    match hash_type {
        4 => {
            let ptr = hash_table.as_ptr() as *const u16;
            unsafe { *ptr.add(hash_value as usize) as u32 }
        }
        5 => {
            let ptr = hash_table.as_ptr() as *const u32;
            unsafe { *ptr.add(hash_value as usize) }
        }
        _ => 0,
    }
}
