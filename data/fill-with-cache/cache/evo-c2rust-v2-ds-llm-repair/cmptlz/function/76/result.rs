pub fn CmptMfAvail(mut mf: Ptr<CmptMfCtx>) -> u32 {
    return (mf.srcLen.cast::<u32>() - mf.readPos).cast();
}
