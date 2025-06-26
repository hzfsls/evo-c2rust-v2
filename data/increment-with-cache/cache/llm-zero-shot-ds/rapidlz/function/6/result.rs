pub fn rapidlz_store_match_len(match_len: u32, token: &mut u8, cur_dest: &mut [u8]) -> usize {
    let cur_dest_anchor = cur_dest.as_ptr() as usize;
    let mut cur_dest_ptr = cur_dest.as_mut_ptr();
    
    if match_len >= RAPIDLZ_MAX_4BIT_VALUE {
        *token += RAPIDLZ_MAX_4BIT_VALUE;
        let mut remaining_len = match_len - RAPIDLZ_MAX_4BIT_VALUE;
        
        unsafe {
            *cur_dest_ptr = RAPIDLZ_MAX_BYTE_VALUE;
            cur_dest_ptr = cur_dest_ptr.add(1);
            
            while remaining_len >= RAPIDLZ_MAX_BYTE_VALUE {
                *cur_dest_ptr = RAPIDLZ_MAX_BYTE_VALUE;
                cur_dest_ptr = cur_dest_ptr.add(1);
                remaining_len -= RAPIDLZ_MAX_BYTE_VALUE;
            }
            
            cur_dest_ptr = cur_dest_ptr.add((remaining_len / RAPIDLZ_MAX_BYTE_VALUE) as usize);
            *cur_dest_ptr = (remaining_len % RAPIDLZ_MAX_BYTE_VALUE) as u8;
            cur_dest_ptr = cur_dest_ptr.add(1);
        }
    } else {
        *token += match_len as u8;
    }
    
    unsafe { cur_dest_ptr.offset_from(cur_dest.as_ptr()) as usize }
}

// Constants (assuming these are defined elsewhere)
const RAPIDLZ_MAX_4BIT_VALUE: u32 = 15;
const RAPIDLZ_MAX_BYTE_VALUE: u32 = 255;
