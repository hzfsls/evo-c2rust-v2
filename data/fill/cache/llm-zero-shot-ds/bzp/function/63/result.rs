use std::mem;

pub fn bzp_mtf_main(mtf: &mut BzpMtfInfo) {
    let mut list = [0u8; BZP_MAX_ALPHA_SIZE];
    let eob;
    let mut num = 0;
    
    bzp_map_input_char(mtf, &mut list, BZP_MAX_ALPHA_SIZE);
    eob = mtf.n_use + 1;
    
    for i in 0..=eob {
        mtf.mtf_freq[i as usize] = 0;
    }
    
    for i in 0..mtf.n_block {
        let mut pos = mtf.map[i as usize] - 1;
        if pos < 0 {
            pos += mtf.n_block;
        }
        let ch = mtf.block[pos as usize];
        
        if ch == list[0] {
            num += 1;
        } else {
            if num > 0 {
                bzp_num_encode(mtf, num);
                num = 0;
            }
            
            let mut pos_ = 1;
            while ch != list[pos_] && pos_ < mtf.n_use as usize {
                pos_ += 1;
            }
            
            for j in (1..=pos_).rev() {
                list[j] = list[j - 1];
            }
            list[0] = ch;
            
            mtf.mtf_v[mtf.n_mtf as usize] = (pos_ + 1) as i32;
            mtf.mtf_freq[(pos_ + 1) as usize] += 1;
            mtf.n_mtf += 1;
        }
    }
    
    if num > 0 {
        bzp_num_encode(mtf, num);
    }
    
    mtf.mtf_v[mtf.n_mtf as usize] = eob;
    mtf.mtf_freq[eob as usize] += 1;
    mtf.n_mtf += 1;
}
