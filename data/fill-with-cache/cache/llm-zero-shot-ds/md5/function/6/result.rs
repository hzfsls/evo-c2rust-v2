use md5::{Md5, Digest};

pub fn vos_md5_calc_ex(output: &mut [u8], input: &[u8]) {
    if output.len() < 16 { // MD5_DIGEST_LEN is 16 bytes
        return;
    }
    
    let mut hasher = Md5::new();
    hasher.update(input);
    let result = hasher.finalize();
    
    output[..16].copy_from_slice(&result[..]);
}
