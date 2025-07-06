pub fn vosSha256HashByBlcMulti(
    mut pucData: Ptr<u8>,
    mut uiLen: u32,
    mut pstCtx: Ptr<VOS_SHA256_CTX>,
) {
    let mut err: errno_t;
    let mut uiBlcLen: u32;
    let mut uiLenTmp: u32 = uiLen;
    let mut pucSrc: Ptr<u8> = pucData;
    uiBlcLen = uiLenTmp.cast::<u32>() / SHA256_BLOCK_SIZE!();
    if uiBlcLen > 0 {
        vosSha256CompressMul(pstCtx, pucSrc, uiBlcLen);
        uiBlcLen *= SHA256_BLOCK_SIZE!();
        pucSrc = pucSrc + uiBlcLen;
        uiLenTmp -= uiBlcLen;
    }
    if uiLenTmp != 0 {
        pstCtx.blocklen = uiLenTmp.cast();
        err = c_memcpy_s!(pstCtx.block, SHA256_BLOCK_SIZE!(), pucSrc, uiLenTmp);
        if err != EOK!() {
            pstCtx.corrupted = 1;
            return;
        }
    }
    return;
}