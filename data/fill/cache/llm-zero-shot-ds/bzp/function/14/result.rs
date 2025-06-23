pub fn bzp_get_dictionary_list(in_data: &mut InDeComdata) -> i32 {
    let mut nin_use = 0;
    let mut use16 = [false; 16];
    let mut in_use = [false; BZP_ASCII_SIZE];
    
    for i in 0..BZP_GROUPS_ASCII {
        use16[i as usize] = bzp_read_bits(BZP_BIT, in_data);
    }
    
    for i in 0..BZP_GROUPS_ASCII {
        if use16[i as usize] {
            for j in 0..BZP_CHARS_PER_GROUP_ASCII {
                in_use[(i * BZP_GROUPS_ASCII + j) as usize] = bzp_read_bits(BZP_BIT, in_data);
            }
        }
    }
    
    for i in 0..BZP_ASCII_SIZE {
        if in_use[i as usize] {
            in_data.list[nin_use as usize] = i as u8;
            nin_use += 1;
        }
    }
    
    nin_use
}
