fn cmpt_mf_avail(mf: &CmptMfCtx) -> u32 {
    mf.src_len - mf.read_pos
}
