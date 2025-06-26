pub fn CmptMfGetPtr(mut mf: Ptr<CmptMfCtx>) -> Ptr<u8> {
    return (mf.srcStart + mf.readPos).cast();
}
