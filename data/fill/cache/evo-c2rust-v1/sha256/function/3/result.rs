pub fn vosSha256HashByBlcMulti(mut pucData: Ptr<u8>, mut uiLen: u32, mut pstCtx: Ptr<VOS_SHA256_CTX>) {
    let mut err: errno_t = Default::default();
    let mut uiBlcLen: u32 = Default::default();
    let mut uiLenTmp: u32 = uiLen.cast();
    let mut pucSrc: Ptr<u8> = pucData.cast();
    uiBlcLen = (uiLenTmp / SHA256_BLOCK_SIZE!()).cast();
    if uiBlcLen > 0 {
        vosSha256CompressMul(pstCtx.cast(), pucSrc.cast(), uiBlcLen.cast());
        uiBlcLen *= SHA256_BLOCK_SIZE!();
        pucSrc += uiBlcLen;
        uiLenTmp -= uiBlcLen;
    }
    if uiLenTmp != 0 {
        pstCtx.blocklen = uiLenTmp.cast();
        err = c_memcpy_s!(pstCtx.block.cast::<Ptr<u8>>(), SHA256_BLOCK_SIZE!(), pucSrc, uiLenTmp).cast();
        if err != EOK!() {
            pstCtx.corrupted = 1;
            return;
        }
    }
    return;
}
