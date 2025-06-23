pub fn vosSha256Hash(mut pucData: Ptr<u8>, mut uiLen: u32, mut pstCtx: Ptr<VOS_SHA256_CTX>) {
    let mut uiBlcLen: u32 = 0;
    let mut uiLenTmp: u32 = uiLen;
    let mut pucSrc: Ptr<u8> = pucData.cast();

    if pucSrc == NULL!() || uiLenTmp == 0 || pstCtx == NULL!() || pstCtx.corrupted == 1 || pstCtx.computed == 1 || vosSha256CtxPrepare(pstCtx.cast(), uiLenTmp.cast()) != SHA256_OK!() {
        return;
    }

    if pstCtx.blocklen != 0 {
        if vosSha256LastPadding(pucSrc.cast(), uiLenTmp.cast(), pstCtx.cast(), c_ref!(uiBlcLen).cast()) == SHA256_OK!() {
            pucSrc += uiBlcLen;
            uiLenTmp -= uiBlcLen;
        } else {
            return;
        }
    }

    vosSha256HashByBlcMulti(pucSrc.cast(), uiLenTmp.cast(), pstCtx.cast());
    return;
}
