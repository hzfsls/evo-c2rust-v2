pub fn vosSha256HashByBlcMulti(mut pucData: Ptr<u8>, mut uiLen: u32, mut pstCtx: Ptr<VOS_SHA256_CTX>) {
    let mut err: errno_t = Default::default();
    let mut uiBlcLen: u32 = Default::default();
    let mut uiLenTmp: u32 = uiLen;
    let mut pucSrc: Ptr<u8> = pucData;

    uiBlcLen = (uiLenTmp / SHA256_BLOCK_SIZE!()).cast();
    if (uiBlcLen > 0).as_bool() {
        vosSha256CompressMul(pstCtx.cast(), pucSrc.cast(), uiBlcLen.cast());
        uiBlcLen *= SHA256_BLOCK_SIZE!();
        pucSrc += uiBlcLen;
        uiLenTmp -= uiBlcLen;
    }

    if (uiLenTmp != 0).as_bool() {
        pstCtx.blocklen = uiLenTmp.cast();
        err = c_memcpy_s!(pstCtx.block, SHA256_BLOCK_SIZE!(), pucSrc, uiLenTmp);
        if (err != EOK!()).as_bool() {
            pstCtx.corrupted = 1;
            return;
        }
    }
    return;
}
