use std::mem;

pub fn bzp_binary_lifting_sort(bwt: &mut BzpBwtInfo) {
    let mut ftab = [0i32; BZP_ASCII_SIZE];
    
    for i in 0..bwt.n_block {
        ftab[bwt.block[i] as usize] += 1;
    }
    
    for i in 1..BZP_ASCII_SIZE {
        ftab[i] += ftab[i - 1];
    }
    
    for i in 0..bwt.n_block {
        let ch = bwt.block[i] as usize;
        ftab[ch] -= 1;
        bwt.sort_block[ftab[ch] as usize] = i as i32;
    }
    
    for i in 0..BZP_ASCII_SIZE {
        bwt.is_start_pos[ftab[i] as usize] = 1;
    }
    
    let mut m = 1;
    let mut sort_flag = true;
    
    while m < bwt.n_block && sort_flag {
        let mut st = 0;
        sort_flag = false;
        
        for i in 0..bwt.n_block {
            if bwt.is_start_pos[i] != 0 {
                st = i;
            }
            let mut pos = bwt.sort_block[i] - m;
            if pos < 0 {
                pos += bwt.n_block as i32;
            }
            bwt.idx[pos as usize] = st as i32;
        }
        
        let mut l = 0;
        let mut r = 1;
        
        while l < bwt.n_block {
            while r < bwt.n_block && bwt.is_start_pos[r] == 0 {
                r += 1;
            }
            r -= 1;
            
            if l < r {
                sort_flag = true;
                bzp_quick_sort(&mut bwt.sort_block, &mut bwt.idx, l as i32, r as i32);
                bzp_update_flag(bwt, l as i32, r as i32);
            }
            
            l = r + 1;
            r = l + 1;
        }
        
        m <<= 1;
    }
}
