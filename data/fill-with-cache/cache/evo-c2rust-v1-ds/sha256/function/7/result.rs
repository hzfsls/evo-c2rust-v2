pub fn vosSha256CompressMul(mut pstCtx: Ptr<VOS_SHA256_CTX>, mut pucInput: Ptr<u8>, mut uiNum: u32) {
    let mut uiNumTmp: u32 = uiNum.cast();
    let mut pucBlock: Ptr<u8> = pucInput.cast();
    while (uiNumTmp.suffix_minus_minus() != 0).as_bool() {
        vosSha256CompressBlock(pstCtx.h.cast(), pucBlock.cast());
        pucBlock += SHA256_BLOCK_SIZE!();
    }
}
