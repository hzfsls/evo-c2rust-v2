pub fn rapidlz_store_literal_len(lit_len: u32, cur_dest: &mut [u8]) -> usize {
    let cur_dest_anchor = cur_dest.as_mut_ptr();
    let mut cur_dest_ptr = cur_dest_anchor;

    if lit_len >= RAPIDLZ_MAX_4BIT_VALUE {
        let mut tmp = lit_len - RAPIDLZ_MAX_4BIT_VALUE;
        unsafe {
            *cur_dest_ptr = RAPIDLZ_MAX_4BIT_VALUE << 4;
            cur_dest_ptr = cur_dest_ptr.add(1);
        }
        while tmp >= RAPIDLZ_MAX_BYTE_VALUE {
            unsafe {
                *cur_dest_ptr = RAPIDLZ_MAX_BYTE_VALUE;
                cur_dest_ptr = cur_dest_ptr.add(1);
            }
            tmp -= RAPIDLZ_MAX_BYTE_VALUE;
        }
        unsafe {
            *cur_dest_ptr = tmp as u8;
            cur_dest_ptr = cur_dest_ptr.add(1);
        }
    } else {
        unsafe {
            *cur_dest_ptr = (lit_len << 4) as u8;
            cur_dest_ptr = cur_dest_ptr.add(1);
        }
    }

    unsafe { cur_dest_ptr.offset_from(cur_dest_anchor) as usize }
}
