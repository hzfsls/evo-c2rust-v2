pub fn cmptlz_mf_gen_hash_table(mf: &mut CmptMfCtx) {
    const POLY32: u32 = 0xEDB88320;
    const CMPT_MF_HASH_TABLE_SIZE: usize = 256; // Assuming standard size if not defined
    
    let hash_root_table = &mut mf.hash_root_table;
    
    for i in 0..CMPT_MF_HASH_TABLE_SIZE {
        let mut value = i as u32;
        for _ in 0..8 {
            if value & 1 != 0 {
                value = (value >> 1) ^ POLY32;
            } else {
                value >>= 1;
            }
        }
        hash_root_table[i] = value;
    }
}
