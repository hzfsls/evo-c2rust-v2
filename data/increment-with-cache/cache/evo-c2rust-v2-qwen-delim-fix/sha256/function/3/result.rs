pub fn vosSha256HashByBlcMulti(mut pucData: Ptr<u8>, mut uiLen: u32, mut pstCtx: Ptr<VOS_SHA256_CTX>) {
    let mut err: errno_t;
    let mut uiBlcLen: u32;
    let mut uiLenTmp: u32 = uiLen.cast();
    let mut pucSrc: Ptr<u8> = pucData.cast();

    uiBlcLen = (uiLenTmp / SHA256_BLOCK_SIZE!()).cast();
    if (uiBlcLen > 0).as_bool() {
        vosSha256CompressMul(pstCtx.cast(), pucSrc.cast(), uiBlcLen.cast());
        uiBlcLen *= SHA256_BLOCK_SIZE!();
        pucSrc += uiBlcLen;
        uiLenTmp -= uiBlcLen;
    }

    if (uiLenTmp != 0).as_bool() {
        pstCtx.blocklen = uiLenTmp.cast();
        err = c_memcpy_s!(pstCtx.block.cast::<Ptr<u8>>(), SHA256_BLOCK_SIZE!(), pucSrc.cast::<Ptr<u8>>(), uiLenTmp.cast());
        if (err != EOK!()).as_bool() {
            pstCtx.corrupted = 1;
            return;
        }
    }
    return;
}