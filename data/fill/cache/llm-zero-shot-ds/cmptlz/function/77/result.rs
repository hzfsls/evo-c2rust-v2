pub fn cmpt_mf_get_ptr(mf: &CmptMfCtx) -> &[u8] {
    &mf.src_start[mf.read_pos..]
}
