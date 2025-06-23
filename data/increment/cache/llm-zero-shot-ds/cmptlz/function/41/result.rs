pub fn cmpt_mem_cmp_by_one_byte(buf1: &[u8], buf2: &[u8], len: u32, limit: u32) -> u32 {
    let mut len_in = len;
    while len_in < limit && len_in < buf1.len() as u32 && len_in < buf2.len() as u32 && buf1[len_in as usize] == buf2[len_in as usize] {
        len_in += 1;
    }
    len_in
}
