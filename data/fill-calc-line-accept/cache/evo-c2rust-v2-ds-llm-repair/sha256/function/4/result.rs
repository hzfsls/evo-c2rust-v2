pub fn vosSha256Hash(mut pucData: Ptr<u8>, mut uiLen: u32, mut pstCtx: Ptr<VOS_SHA256_CTX>) {
    let mut uiBlcLen: u32 = 0;
    let mut uiLenTmp: u32 = uiLen;
    let mut pucSrc: Ptr<u8> = pucData.cast();
    if (pucSrc == NULL!()).as_bool() || (uiLenTmp == 0).as_bool() || (pstCtx == NULL!()).as_bool() || (pstCtx.corrupted == 1).as_bool() || (pstCtx.computed == 1).as_bool() || (vosSha256CtxPrepare(pstCtx.cast(), uiLenTmp.cast()) != SHA256_OK!()).as_bool() {
        return;
    }
    if (pstCtx.blocklen != 0).as_bool() {
        if (vosSha256LastPadding(pucSrc.cast(), uiLenTmp.cast(), pstCtx.cast(), c_ref!(uiBlcLen).cast()) == SHA256_OK!()).as_bool() {
            pucSrc += uiBlcLen;
            uiLenTmp -= uiBlcLen;
        } else {
            return;
        }
    }
    vosSha256HashByBlcMulti(pucSrc.cast(), uiLenTmp.cast(), pstCtx.cast());
    return;
}
