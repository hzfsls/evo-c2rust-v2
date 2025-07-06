pub fn CmptMfAvail(mut mf: Ptr<CmptMfCtx>) -> u32 {
    return mf.srcLen as u32 - mf.readPos;
}