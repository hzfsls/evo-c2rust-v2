#[inline]
fn rapidlz_put_pos_on_table(pos: u32, hash_value: u32, hash_table: &mut [u8], hash_type: u8) {
    match hash_type {
        4 => {
            let table_ptr = hash_table.as_mut_ptr() as *mut u16;
            unsafe {
                *table_ptr.add(hash_value as usize) = pos as u16;
            }
        }
        5 => {
            let table_ptr = hash_table.as_mut_ptr() as *mut u32;
            unsafe {
                *table_ptr.add(hash_value as usize) = pos;
            }
        }
        _ => {}
    }
}
