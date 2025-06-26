pub fn cmpt_mem_cmp_len(buf1: &[u8], buf2: &[u8], len: u32, limit: u32) -> u32 {
    cmpt_mem_cmp_by_one_byte(buf1, buf2, len, limit)
}
