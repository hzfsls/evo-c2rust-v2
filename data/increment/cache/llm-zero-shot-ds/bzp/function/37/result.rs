use std::ptr::write_bytes;

pub fn bzp_bwt_decode(bwt: &mut BzpBwtDecodeInfo) {
    let mut ftab = [0i32; 257];
    
    // Initialize ftab with zeros (equivalent to memset_s)
    unsafe {
        write_bytes(ftab.as_mut_ptr(), 0, ftab.len());
    }
    
    // First pass: count occurrences of each byte value
    for &byte in &bwt.block[..bwt.n_block as usize] {
        ftab[(byte + 1) as usize] += 1;
    }
    
    // Second pass: compute cumulative sums
    for i in 1..=BZP_ASCII_SIZE {
        ftab[i] += ftab[i - 1];
    }
    
    // Third pass: build the sorted array
    for (i, &byte) in bwt.block[..bwt.n_block as usize].iter().enumerate() {
        let idx = ftab[byte as usize] as usize;
        bwt.sorted[idx] = i as i32;
        ftab[byte as usize] += 1;
    }
    
    // Reconstruct the original data
    let mut cnt = 0;
    let mut pos = bwt.ori_ptr;
    while cnt < bwt.n_block {
        pos = bwt.sorted[pos as usize];
        let byte = bwt.block[pos as usize];
        bwt.de_code[cnt as usize] = byte;
        cnt += 1;
    }
}
