pub fn vosSha256Hash(mut pucData: Ptr<u8>, mut uiLen: u32, mut pstCtx: Ptr<VOS_SHA256_CTX>) {
    let mut uiBlcLen: u32 = 0;
    let mut uiLenTmp: u32 = uiLen;
    let mut pucSrc: Ptr<u8> = pucData;
    if pucSrc == NULL!()
        || uiLenTmp == 0
        || pstCtx == NULL!()
        || pstCtx.corrupted == 1
        || pstCtx.computed == 1
        || vosSha256CtxPrepare(pstCtx, uiLenTmp) != SHA256_OK!()
    {
        return;
    }
    if pstCtx.blocklen != 0 {
        if vosSha256LastPadding(pucSrc, uiLenTmp, pstCtx, c_ref!(uiBlcLen)) == SHA256_OK!() {
            pucSrc = pucSrc + uiBlcLen;
            uiLenTmp -= uiBlcLen;
        } else {
            return;
        }
    }
    vosSha256HashByBlcMulti(pucSrc, uiLenTmp, pstCtx);
    return;
}