pub fn cmpt_mf_avail(mf: &CmptMfCtx) -> u32 {
    mf.srcLen - mf.readPos
}
