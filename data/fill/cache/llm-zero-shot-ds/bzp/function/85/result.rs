use std::ptr::write_bytes;

pub fn bzp_bwt_decode(bwt: &mut BzpBwtDecodeInfo) {
    let mut ftab = [0i32; 257];
    
    // Initialize ftab with zeros (equivalent to memset_s)
    unsafe {
        write_bytes(ftab.as_mut_ptr(), 0, ftab.len());
    }
    
    // First pass: count occurrences of each byte value + 1
    for i in 0..bwt.n_block {
        let byte = bwt.block[i] as usize;
        ftab[byte + 1] += 1;
    }
    
    // Second pass: compute cumulative sums
    for i in 1..=BZP_ASCII_SIZE {
        ftab[i] += ftab[i - 1];
    }
    
    // Third pass: build the sorted array
    for i in 0..bwt.n_block {
        let ch = bwt.block[i] as usize;
        bwt.sorted[ftab[ch] as usize] = i;
        ftab[ch] += 1;
    }
    
    // Reconstruct the original data
    let mut cnt = 0;
    let mut pos = bwt.ori_ptr;
    while cnt < bwt.n_block {
        pos = bwt.sorted[pos as usize];
        let ch = bwt.block[pos as usize];
        bwt.de_code[cnt as usize] = ch;
        cnt += 1;
    }
}
