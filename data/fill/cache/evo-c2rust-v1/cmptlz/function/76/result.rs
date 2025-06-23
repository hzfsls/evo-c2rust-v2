pub fn CmptMfAvail(mut mf: Ptr<CmptMfCtx>) -> u32 {
    return (mf.srcLen - mf.readPos).cast();
}
